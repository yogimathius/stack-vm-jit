import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react'
import { VMState, VMProgram, VMExecution, JITProfile, PerformanceMetrics } from '../types/vm'
import { createMockVM } from '../utils/mockVM'

interface VMContextType {
  vmState: VMState
  currentProgram: VMProgram | null
  execution: VMExecution | null
  jitProfile: JITProfile | null
  metrics: PerformanceMetrics | null
  isRunning: boolean
  
  loadProgram: (program: VMProgram) => void
  runProgram: () => Promise<void>
  stepProgram: () => Promise<void>
  pauseProgram: () => void
  resetProgram: () => void
  compileCode: (code: string, language: 'assembly' | 'expression') => Promise<VMProgram>
}

const VMContext = createContext<VMContextType | undefined>(undefined)

export function VMProvider({ children }: { children: ReactNode }) {
  const [vmState, setVMState] = useState<VMState>(createInitialVMState())
  const [currentProgram, setCurrentProgram] = useState<VMProgram | null>(null)
  const [execution, setExecution] = useState<VMExecution | null>(null)
  const [jitProfile, setJitProfile] = useState<JITProfile | null>(null)
  const [metrics, setMetrics] = useState<PerformanceMetrics | null>(null)
  const [isRunning, setIsRunning] = useState(false)

  const loadProgram = useCallback((program: VMProgram) => {
    setCurrentProgram(program)
    setVMState(createInitialVMState())
    setExecution(null)
    setJitProfile(null)
    setMetrics(null)
  }, [])

  const runProgram = useCallback(async () => {
    if (!currentProgram) return
    
    setIsRunning(true)
    const mockVM = createMockVM()
    
    try {
      const result = await mockVM.run(currentProgram)
      setExecution(result.execution)
      setVMState(result.execution.state)
      setJitProfile(result.jitProfile)
      setMetrics(result.metrics)
    } catch (error) {
      console.error('VM execution error:', error)
      setVMState(prev => ({ ...prev, error: String(error), isRunning: false, isHalted: true }))
    } finally {
      setIsRunning(false)
    }
  }, [currentProgram])

  const stepProgram = useCallback(async () => {
    if (!currentProgram) return
    
    const mockVM = createMockVM()
    try {
      const result = await mockVM.step(currentProgram, vmState)
      setExecution(result.execution)
      setVMState(result.execution.state)
    } catch (error) {
      console.error('VM step error:', error)
      setVMState(prev => ({ ...prev, error: String(error) }))
    }
  }, [currentProgram, vmState])

  const pauseProgram = useCallback(() => {
    setIsRunning(false)
    setVMState(prev => ({ ...prev, isRunning: false }))
  }, [])

  const resetProgram = useCallback(() => {
    setVMState(createInitialVMState())
    setExecution(null)
    setIsRunning(false)
  }, [])

  const compileCode = useCallback(async (code: string, language: 'assembly' | 'expression'): Promise<VMProgram> => {
    const mockVM = createMockVM()
    return mockVM.compile(code, language)
  }, [])

  const value = {
    vmState,
    currentProgram,
    execution,
    jitProfile,
    metrics,
    isRunning,
    loadProgram,
    runProgram,
    stepProgram,
    pauseProgram,
    resetProgram,
    compileCode,
  }

  return <VMContext.Provider value={value}>{children}</VMContext.Provider>
}

export function useVM() {
  const context = useContext(VMContext)
  if (context === undefined) {
    throw new Error('useVM must be used within a VMProvider')
  }
  return context
}

function createInitialVMState(): VMState {
  return {
    stack: [],
    memory: new Map(),
    programCounter: 0,
    callStack: [],
    isRunning: false,
    isHalted: false,
    instructionCount: 0,
    executionTime: 0,
  }
}