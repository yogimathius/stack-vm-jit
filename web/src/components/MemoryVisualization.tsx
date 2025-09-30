import React from 'react'
import { VMValue } from '../types/vm'

interface MemoryVisualizationProps {
  memory: Map<number, VMValue>
}

export default function MemoryVisualization({ memory }: MemoryVisualizationProps) {
  const memoryArray = Array.from(memory.entries()).sort((a, b) => a[0] - b[0])

  if (memoryArray.length === 0) {
    return (
      <div className="text-center py-12">
        <div className="w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-lg flex items-center justify-center">
          <span className="text-2xl text-gray-400">💾</span>
        </div>
        <h3 className="text-lg font-medium text-gray-900 mb-2">Memory Empty</h3>
        <p className="text-gray-600">
          Use LOAD/STORE instructions to see memory usage
        </p>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold text-gray-900">Heap Memory</h3>
        <span className="px-2 py-1 bg-green-100 text-green-800 rounded-full text-sm font-medium">
          {memoryArray.length} allocated
        </span>
      </div>

      <div className="grid grid-cols-4 gap-2">
        {memoryArray.map(([address, value]) => (
          <div
            key={address}
            className="memory-cell p-2 rounded border text-center hover:shadow-md transition-all"
          >
            <div className="text-xs text-gray-500 mb-1">0x{address.toString(16).padStart(4, '0')}</div>
            <div className="font-mono text-sm font-semibold">
              {value.type === 'Integer' ? value.value : `[${value.type}]`}
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}