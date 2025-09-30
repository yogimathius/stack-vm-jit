import React, { useState, useEffect } from 'react'
import { Play, Pause, Square, RotateCcw, StepForward, Zap, Clock, MemoryStick } from 'lucide-react'
import { useVM } from '../contexts/VMContext'
import CodeEditor from '../components/CodeEditor'
import StackVisualization from '../components/StackVisualization'
import MemoryVisualization from '../components/MemoryVisualization'
import InstructionViewer from '../components/InstructionViewer'
import PerformancePanel from '../components/PerformancePanel'
import JITProfiler from '../components/JITProfiler'

const defaultCode = `; Calculate (5 + 3) * 2
PUSH 5      ; Push 5 onto stack
PUSH 3      ; Push 3 onto stack  
ADD         ; Add top two values
PUSH 2      ; Push 2 onto stack
MUL         ; Multiply top two values
HALT        ; Stop execution`

const examples = [
  {
    name: 'Basic Arithmetic',
    code: defaultCode
  },
  {
    name: 'Fibonacci Sequence',
    code: `; Calculate Fibonacci(5)
PUSH 0      ; fib(0) = 0
PUSH 1      ; fib(1) = 1
PUSH 5      ; n = 5
loop:
DUP         ; Duplicate n
PUSH 2      ; Compare with 2
LT          ; n < 2?
JT end      ; Jump to end if true
DUP         ; Duplicate n
PUSH 1      ; n - 1
SUB
PUSH 2      ; n - 2  
SUB
ADD         ; fib(n-1) + fib(n-2)
SWAP        ; Swap for next iteration
PUSH 1      ; n - 1
SUB
JMP loop    ; Continue loop
end:
HALT`
  },
  {
    name: 'Stack Manipulation',
    code: `; Demonstrate stack operations
PUSH 10
PUSH 20
PUSH 30
DUP         ; Duplicate top (30)
SWAP        ; Swap top two (30, 30, 20, 10)
POP         ; Remove top
ADD         ; Add top two
HALT`
  }
]

export default function Playground() {
  const { 
    vmState, 
    currentProgram, 
    isRunning, 
    execution,
    jitProfile,
    metrics,
    loadProgram, 
    runProgram, 
    stepProgram, 
    pauseProgram, 
    resetProgram,
    compileCode 
  } = useVM()
  
  const [code, setCode] = useState(defaultCode)
  const [selectedExample, setSelectedExample] = useState(0)
  const [compileError, setCompileError] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState<'stack' | 'memory' | 'performance' | 'jit'>('stack')

  // Auto-compile when code changes
  useEffect(() => {
    const compileTimer = setTimeout(async () => {
      try {
        const program = await compileCode(code, 'assembly')
        loadProgram(program)
        setCompileError(null)
      } catch (error) {
        setCompileError(String(error))
      }
    }, 500)

    return () => clearTimeout(compileTimer)
  }, [code, compileCode, loadProgram])

  const handleExampleChange = (index: number) => {
    setSelectedExample(index)
    setCode(examples[index].code)
  }

  const getVMStateColor = () => {
    if (vmState.error) return 'text-red-600'
    if (vmState.isRunning) return 'text-green-600'
    if (vmState.isHalted) return 'text-blue-600'
    return 'text-gray-600'
  }

  const getVMStateText = () => {
    if (vmState.error) return 'Error'
    if (vmState.isRunning) return 'Running'
    if (vmState.isHalted) return 'Halted'
    return 'Ready'
  }

  return (
    <div className="vm-container space-y-6">
      {/* Header */}
      <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-4">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">VM Playground</h1>
          <p className="text-gray-600 mt-1">
            Interactive virtual machine with real-time visualization and JIT profiling
          </p>
        </div>
        
        {/* VM Status */}
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2">
            <div className={`w-3 h-3 rounded-full ${vmState.isRunning ? 'bg-green-500 animate-pulse' : vmState.error ? 'bg-red-500' : 'bg-gray-400'}`} />
            <span className={`font-medium ${getVMStateColor()}`}>
              {getVMStateText()}
            </span>
          </div>
          
          {vmState.instructionCount > 0 && (
            <div className="text-sm text-gray-600">
              PC: {vmState.programCounter} | Instructions: {vmState.instructionCount}
            </div>
          )}
        </div>
      </div>

      {/* Controls */}
      <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between bg-white p-4 rounded-lg border">
        <div className="flex flex-wrap gap-2">
          <button
            onClick={isRunning ? pauseProgram : runProgram}
            disabled={!currentProgram || !!compileError}
            className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isRunning ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
            {isRunning ? 'Pause' : 'Run'}
          </button>
          
          <button
            onClick={stepProgram}
            disabled={!currentProgram || isRunning || vmState.isHalted || !!compileError}
            className="flex items-center gap-2 px-4 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <StepForward className="w-4 h-4" />
            Step
          </button>
          
          <button
            onClick={resetProgram}
            disabled={isRunning}
            className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <RotateCcw className="w-4 h-4" />
            Reset
          </button>
        </div>

        {/* Example Selector */}
        <div className="flex items-center gap-2">
          <label className="text-sm font-medium text-gray-700">Examples:</label>
          <select
            value={selectedExample}
            onChange={(e) => handleExampleChange(Number(e.target.value))}
            className="px-3 py-1 border border-gray-300 rounded-md text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          >
            {examples.map((example, index) => (
              <option key={index} value={index}>
                {example.name}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Error Display */}
      {(compileError || vmState.error) && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <div className="flex items-center gap-2 text-red-800">
            <Square className="w-4 h-4" />
            <span className="font-medium">Error</span>
          </div>
          <p className="text-red-700 mt-1">{compileError || vmState.error}</p>
        </div>
      )}

      {/* Main Layout */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Code Editor */}
        <div className="space-y-4">
          <div className="bg-white rounded-lg border overflow-hidden">
            <div className="bg-gray-50 px-4 py-2 border-b">
              <h3 className="font-semibold text-gray-900">Assembly Code</h3>
            </div>
            <CodeEditor
              value={code}
              onChange={setCode}
              language="assembly"
              currentLine={vmState.programCounter}
            />
          </div>

          {/* Instructions */}
          {currentProgram && (
            <InstructionViewer
              instructions={currentProgram.instructions}
              currentPC={vmState.programCounter}
              jitProfile={jitProfile}
            />
          )}
        </div>

        {/* Visualizations */}
        <div className="space-y-4">
          {/* Tabs */}
          <div className="bg-white rounded-lg border overflow-hidden">
            <div className="flex border-b">
              {[
                { key: 'stack', label: 'Stack', icon: Square },
                { key: 'memory', label: 'Memory', icon: MemoryStick },
                { key: 'performance', label: 'Performance', icon: Clock },
                { key: 'jit', label: 'JIT Profile', icon: Zap },
              ].map((tab) => (
                <button
                  key={tab.key}
                  onClick={() => setActiveTab(tab.key as any)}
                  className={`flex items-center gap-2 px-4 py-3 text-sm font-medium transition-colors ${
                    activeTab === tab.key
                      ? 'text-blue-600 bg-blue-50 border-b-2 border-blue-600'
                      : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
                  }`}
                >
                  <tab.icon className="w-4 h-4" />
                  {tab.label}
                </button>
              ))}
            </div>

            <div className="p-4">
              {activeTab === 'stack' && (
                <StackVisualization stack={vmState.stack} />
              )}
              {activeTab === 'memory' && (
                <MemoryVisualization memory={vmState.memory} />
              )}
              {activeTab === 'performance' && (
                <PerformancePanel 
                  metrics={metrics}
                  executionTrace={execution?.trace || []}
                />
              )}
              {activeTab === 'jit' && (
                <JITProfiler 
                  profile={jitProfile}
                  instructions={currentProgram?.instructions || []}
                />
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}