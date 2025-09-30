import React from 'react'
import { VMInstruction, JITProfile } from '../types/vm'
import { Zap, Target } from 'lucide-react'

interface InstructionViewerProps {
  instructions: VMInstruction[]
  currentPC: number
  jitProfile?: JITProfile | null
}

export default function InstructionViewer({ instructions, currentPC, jitProfile }: InstructionViewerProps) {
  const getInstructionClass = (index: number) => {
    const baseClass = 'instruction-line p-2 font-mono text-sm transition-all'
    
    if (index === currentPC) {
      return `${baseClass} current bg-blue-100 border-l-4 border-blue-500 font-semibold`
    }
    
    if (index < currentPC) {
      return `${baseClass} executed bg-green-50 text-green-800`
    }
    
    const isHotSpot = jitProfile?.hotSpots.some(h => h.pc === index && h.count > 5)
    const isCandidate = jitProfile?.compilationCandidates.includes(index)
    
    if (isCandidate) {
      return `${baseClass} jit-compiled`
    }
    
    if (isHotSpot) {
      return `${baseClass} jit-hot-spot`
    }
    
    return `${baseClass} hover:bg-gray-50`
  }

  return (
    <div className="bg-white rounded-lg border overflow-hidden">
      <div className="bg-gray-50 px-4 py-2 border-b flex items-center justify-between">
        <h3 className="font-semibold text-gray-900">Instructions</h3>
        <div className="flex items-center gap-2 text-xs">
          <div className="flex items-center gap-1">
            <div className="w-2 h-2 bg-blue-500 rounded"></div>
            <span>Current</span>
          </div>
          <div className="flex items-center gap-1">
            <div className="w-2 h-2 bg-yellow-500 rounded"></div>
            <span>Hot Spot</span>
          </div>
          <div className="flex items-center gap-1">
            <div className="w-2 h-2 bg-purple-500 rounded"></div>
            <span>JIT Compiled</span>
          </div>
        </div>
      </div>
      
      <div className="max-h-96 overflow-y-auto">
        {instructions.map((instruction, index) => {
          const hotSpot = jitProfile?.hotSpots.find(h => h.pc === index)
          const isCandidate = jitProfile?.compilationCandidates.includes(index)
          
          return (
            <div key={index} className={getInstructionClass(index)}>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <span className="text-gray-500 w-8 text-right">{index}</span>
                  <span className="font-semibold text-blue-700">{instruction.opcode}</span>
                  {instruction.operand !== undefined && (
                    <span className="text-gray-700">{instruction.operand}</span>
                  )}
                </div>
                
                <div className="flex items-center gap-2">
                  {hotSpot && (
                    <div className="flex items-center gap-1 text-xs text-orange-700">
                      <Target className="w-3 h-3" />
                      <span>{hotSpot.count}</span>
                    </div>
                  )}
                  {isCandidate && (
                    <div className="flex items-center gap-1 text-xs text-purple-700">
                      <Zap className="w-3 h-3" />
                      <span>JIT</span>
                    </div>
                  )}
                </div>
              </div>
            </div>
          )
        })}
      </div>
    </div>
  )
}