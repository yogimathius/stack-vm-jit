import React from 'react'
import { JITProfile, VMInstruction } from '../types/vm'
import { Zap, Target, TrendingUp } from 'lucide-react'

interface JITProfilerProps {
  profile: JITProfile | null
  instructions: VMInstruction[]
}

export default function JITProfiler({ profile, instructions }: JITProfilerProps) {
  if (!profile) {
    return (
      <div className="text-center py-8">
        <Zap className="w-12 h-12 text-gray-400 mx-auto mb-3" />
        <p className="text-gray-600">Run a program to see JIT compilation profile</p>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <h3 className="text-lg font-semibold text-gray-900">JIT Compilation Profile</h3>
      
      {/* Hot Spots */}
      <div className="space-y-3">
        <div className="flex items-center gap-2">
          <Target className="w-5 h-5 text-orange-600" />
          <h4 className="font-medium text-gray-900">Hot Spots</h4>
        </div>
        
        {profile.hotSpots.length === 0 ? (
          <p className="text-gray-600 text-sm">No hot spots detected</p>
        ) : (
          <div className="space-y-2">
            {profile.hotSpots.slice(0, 5).map((hotSpot) => {
              const instruction = instructions[hotSpot.pc]
              return (
                <div key={hotSpot.pc} className="bg-orange-50 border border-orange-200 rounded-lg p-3">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <span className="font-mono text-sm bg-orange-200 px-2 py-1 rounded">
                        PC {hotSpot.pc}
                      </span>
                      {instruction && (
                        <span className="font-mono text-sm">
                          {instruction.opcode} {instruction.operand || ''}
                        </span>
                      )}
                    </div>
                    <div className="text-right">
                      <div className="text-sm font-semibold text-orange-800">
                        {hotSpot.count} executions
                      </div>
                      <div className="text-xs text-orange-600">
                        {hotSpot.percentage.toFixed(1)}% of total
                      </div>
                    </div>
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>

      {/* Compilation Candidates */}
      <div className="space-y-3">
        <div className="flex items-center gap-2">
          <Zap className="w-5 h-5 text-purple-600" />
          <h4 className="font-medium text-gray-900">JIT Compilation Candidates</h4>
        </div>
        
        {profile.compilationCandidates.length === 0 ? (
          <p className="text-gray-600 text-sm">No compilation candidates identified</p>
        ) : (
          <div className="grid grid-cols-1 gap-2">
            {profile.compilationCandidates.map((pc) => {
              const instruction = instructions[pc]
              const hotSpot = profile.hotSpots.find(h => h.pc === pc)
              
              return (
                <div key={pc} className="bg-purple-50 border border-purple-200 rounded-lg p-3">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <span className="font-mono text-sm bg-purple-200 px-2 py-1 rounded">
                        PC {pc}
                      </span>
                      {instruction && (
                        <span className="font-mono text-sm">
                          {instruction.opcode} {instruction.operand || ''}
                        </span>
                      )}
                    </div>
                    {hotSpot && (
                      <div className="text-xs text-purple-600">
                        {hotSpot.count} calls
                      </div>
                    )}
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>

      {/* Optimizations */}
      <div className="space-y-3">
        <div className="flex items-center gap-2">
          <TrendingUp className="w-5 h-5 text-green-600" />
          <h4 className="font-medium text-gray-900">Potential Optimizations</h4>
        </div>
        
        {profile.optimizations.length === 0 ? (
          <p className="text-gray-600 text-sm">No optimizations suggested</p>
        ) : (
          <div className="space-y-2">
            {profile.optimizations.map((opt, index) => (
              <div key={index} className="bg-green-50 border border-green-200 rounded-lg p-3">
                <div className="flex items-start gap-3">
                  <span className="font-mono text-sm bg-green-200 px-2 py-1 rounded">
                    PC {opt.location}
                  </span>
                  <div>
                    <div className="font-medium text-green-800">{opt.type}</div>
                    <div className="text-sm text-green-700">{opt.description}</div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}