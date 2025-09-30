import { VMProgram, VMInstruction, VMValue, VMExecution, VMState, JITProfile, PerformanceMetrics, ExecutionStep } from '../types/vm'

export function createMockVM() {
  return {
    async compile(code: string, language: 'assembly' | 'expression'): Promise<VMProgram> {
      // Mock compilation - parse simple assembly or expressions
      const instructions: VMInstruction[] = []
      const constants: VMValue[] = []
      
      if (language === 'assembly') {
        const lines = code.trim().split('\n')
        lines.forEach((line, index) => {
          const trimmed = line.trim()
          if (trimmed && !trimmed.startsWith(';')) {
            const parts = trimmed.split(/\s+/)
            const opcode = parts[0].toUpperCase()
            const operand = parts[1] ? (isNaN(Number(parts[1])) ? parts[1] : Number(parts[1])) : undefined
            
            instructions.push({
              opcode,
              operand,
              address: index
            })
          }
        })
      } else {
        // Simple expression compilation
        instructions.push(
          { opcode: 'PUSH', operand: 5, address: 0 },
          { opcode: 'PUSH', operand: 3, address: 1 },
          { opcode: 'ADD', address: 2 },
          { opcode: 'PUSH', operand: 2, address: 3 },
          { opcode: 'MUL', address: 4 },
          { opcode: 'HALT', address: 5 }
        )
      }

      return {
        instructions,
        constants,
        sourceCode: code,
        language
      }
    },

    async run(program: VMProgram): Promise<{
      execution: VMExecution
      jitProfile: JITProfile
      metrics: PerformanceMetrics
    }> {
      const state: VMState = {
        stack: [],
        memory: new Map(),
        programCounter: 0,
        callStack: [],
        isRunning: true,
        isHalted: false,
        instructionCount: 0,
        executionTime: 0
      }

      const trace: ExecutionStep[] = []
      const hotSpots = new Map<number, number>()
      const startTime = performance.now()

      // Simulate execution
      while (state.programCounter < program.instructions.length && !state.isHalted) {
        const instruction = program.instructions[state.programCounter]
        const stateBefore = { ...state, stack: [...state.stack] }
        
        // Track hot spots
        hotSpots.set(state.programCounter, (hotSpots.get(state.programCounter) || 0) + 1)
        
        // Execute instruction
        await this.executeInstruction(instruction, state)
        
        const stateAfter = { ...state, stack: [...state.stack] }
        
        trace.push({
          instruction,
          stateBefore,
          stateAfter,
          timestamp: performance.now(),
          executionTime: performance.now() - startTime
        })

        state.instructionCount++
        
        if (instruction.opcode !== 'HALT') {
          state.programCounter++
        }

        // Simulate execution delay
        await new Promise(resolve => setTimeout(resolve, 50))
      }

      state.executionTime = performance.now() - startTime
      state.isRunning = false

      // Generate JIT profile
      const hotSpotsArray = Array.from(hotSpots.entries())
        .map(([pc, count]) => ({
          pc,
          count,
          percentage: (count / state.instructionCount) * 100
        }))
        .sort((a, b) => b.count - a.count)

      const jitProfile: JITProfile = {
        hotSpots: hotSpotsArray,
        compilationCandidates: hotSpotsArray.filter(h => h.count > 10).map(h => h.pc),
        optimizations: [
          { type: 'Loop Unrolling', location: 2, description: 'Unroll arithmetic loop for better performance' },
          { type: 'Constant Folding', location: 0, description: 'Fold constant expressions at compile time' }
        ]
      }

      const metrics: PerformanceMetrics = {
        instructionsPerSecond: state.instructionCount / (state.executionTime / 1000),
        memoryUsage: state.memory.size * 8, // Rough estimate
        gcCollections: 0,
        jitCompilations: jitProfile.compilationCandidates.length,
        executionTime: state.executionTime,
        totalInstructions: state.instructionCount
      }

      return {
        execution: {
          state,
          currentInstruction: program.instructions[state.programCounter - 1],
          trace
        },
        jitProfile,
        metrics
      }
    },

    async step(program: VMProgram, currentState: VMState): Promise<{
      execution: VMExecution
    }> {
      const state = { ...currentState }
      
      if (state.programCounter < program.instructions.length && !state.isHalted) {
        const instruction = program.instructions[state.programCounter]
        await this.executeInstruction(instruction, state)
        state.instructionCount++
        
        if (instruction.opcode !== 'HALT') {
          state.programCounter++
        }
      }

      return {
        execution: {
          state,
          currentInstruction: program.instructions[state.programCounter],
          trace: [] // Simplified for step mode
        }
      }
    },

    async executeInstruction(instruction: VMInstruction, state: VMState): Promise<void> {
      switch (instruction.opcode) {
        case 'PUSH':
          state.stack.push({ type: 'Integer', value: instruction.operand })
          break
        
        case 'POP':
          state.stack.pop()
          break
        
        case 'ADD':
          const b = state.stack.pop()
          const a = state.stack.pop()
          if (a && b) {
            state.stack.push({ type: 'Integer', value: a.value + b.value })
          }
          break
        
        case 'SUB':
          const b2 = state.stack.pop()
          const a2 = state.stack.pop()
          if (a2 && b2) {
            state.stack.push({ type: 'Integer', value: a2.value - b2.value })
          }
          break
        
        case 'MUL':
          const b3 = state.stack.pop()
          const a3 = state.stack.pop()
          if (a3 && b3) {
            state.stack.push({ type: 'Integer', value: a3.value * b3.value })
          }
          break
        
        case 'DIV':
          const b4 = state.stack.pop()
          const a4 = state.stack.pop()
          if (a4 && b4 && b4.value !== 0) {
            state.stack.push({ type: 'Integer', value: Math.floor(a4.value / b4.value) })
          }
          break
        
        case 'DUP':
          const top = state.stack[state.stack.length - 1]
          if (top) {
            state.stack.push({ ...top })
          }
          break
        
        case 'SWAP':
          if (state.stack.length >= 2) {
            const a = state.stack.pop()!
            const b = state.stack.pop()!
            state.stack.push(a)
            state.stack.push(b)
          }
          break
        
        case 'HALT':
          state.isHalted = true
          state.isRunning = false
          break
        
        default:
          console.warn(`Unknown instruction: ${instruction.opcode}`)
      }
    }
  }
}