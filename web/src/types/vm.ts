export interface VMInstruction {
  opcode: string
  operand?: number | string
  address: number
}

export interface VMValue {
  type: 'Integer' | 'Float' | 'Boolean' | 'String' | 'Object' | 'Array'
  value: any
}

export interface VMState {
  stack: VMValue[]
  memory: Map<number, VMValue>
  programCounter: number
  callStack: number[]
  isRunning: boolean
  isHalted: boolean
  error?: string
  instructionCount: number
  executionTime: number
}

export interface VMProgram {
  instructions: VMInstruction[]
  constants: VMValue[]
  sourceCode: string
  language: 'assembly' | 'expression'
}

export interface VMExecution {
  state: VMState
  currentInstruction?: VMInstruction
  trace: ExecutionStep[]
}

export interface ExecutionStep {
  instruction: VMInstruction
  stateBefore: Partial<VMState>
  stateAfter: Partial<VMState>
  timestamp: number
  executionTime: number
}

export interface JITProfile {
  hotSpots: Array<{
    pc: number
    count: number
    percentage: number
  }>
  compilationCandidates: number[]
  optimizations: Array<{
    type: string
    location: number
    description: string
  }>
}

export interface PerformanceMetrics {
  instructionsPerSecond: number
  memoryUsage: number
  gcCollections: number
  jitCompilations: number
  executionTime: number
  totalInstructions: number
}