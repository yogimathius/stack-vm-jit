import React from 'react'
import { VMValue } from '../types/vm'

interface StackVisualizationProps {
  stack: VMValue[]
}

export default function StackVisualization({ stack }: StackVisualizationProps) {
  const getValueDisplay = (value: VMValue) => {
    switch (value.type) {
      case 'Integer':
        return value.value.toString()
      case 'Float':
        return value.value.toFixed(2)
      case 'Boolean':
        return value.value ? 'true' : 'false'
      case 'String':
        return `"${value.value}"`
      case 'Object':
        return '[Object]'
      case 'Array':
        return `[${value.value.length}]`
      default:
        return String(value.value)
    }
  }

  const getValueColor = (type: string) => {
    switch (type) {
      case 'Integer':
        return 'bg-blue-100 text-blue-800 border-blue-200'
      case 'Float':
        return 'bg-green-100 text-green-800 border-green-200'
      case 'Boolean':
        return 'bg-purple-100 text-purple-800 border-purple-200'
      case 'String':
        return 'bg-yellow-100 text-yellow-800 border-yellow-200'
      case 'Object':
        return 'bg-red-100 text-red-800 border-red-200'
      case 'Array':
        return 'bg-indigo-100 text-indigo-800 border-indigo-200'
      default:
        return 'bg-gray-100 text-gray-800 border-gray-200'
    }
  }

  if (stack.length === 0) {
    return (
      <div className="text-center py-12">
        <div className="w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-lg flex items-center justify-center">
          <span className="text-2xl text-gray-400">📚</span>
        </div>
        <h3 className="text-lg font-medium text-gray-900 mb-2">Stack Empty</h3>
        <p className="text-gray-600">
          Execute instructions to see values appear on the stack
        </p>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold text-gray-900">Operand Stack</h3>
        <span className="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium">
          {stack.length} item{stack.length !== 1 ? 's' : ''}
        </span>
      </div>

      <div className="space-y-2">
        <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">
          Top → Bottom
        </div>
        
        {/* Stack items displayed from top to bottom */}
        <div className="space-y-2">
          {[...stack].reverse().map((item, index) => {
            const actualIndex = stack.length - 1 - index
            const isTop = index === 0
            
            return (
              <div
                key={actualIndex}
                className={`stack-item p-3 border-2 rounded-lg transition-all duration-300 ${
                  getValueColor(item.type)
                } ${isTop ? 'ring-2 ring-blue-300 shadow-lg' : ''}`}
                style={{
                  transform: isTop ? 'scale(1.02)' : 'scale(1)',
                }}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <div className="flex items-center justify-center w-8 h-8 bg-white rounded-full text-sm font-bold">
                      {actualIndex}
                    </div>
                    <div>
                      <div className="font-mono text-lg font-semibold">
                        {getValueDisplay(item)}
                      </div>
                      <div className="text-xs opacity-75">
                        {item.type}
                      </div>
                    </div>
                  </div>
                  
                  {isTop && (
                    <div className="flex items-center text-xs font-medium">
                      <span className="bg-white bg-opacity-50 px-2 py-1 rounded">
                        TOP
                      </span>
                    </div>
                  )}
                </div>
              </div>
            )
          })}
        </div>
      </div>

      {/* Stack operations hint */}
      <div className="mt-6 p-3 bg-gray-50 rounded-lg">
        <h4 className="text-sm font-medium text-gray-900 mb-2">Stack Operations</h4>
        <div className="text-xs text-gray-600 space-y-1">
          <div><code className="font-mono bg-gray-200 px-1 rounded">PUSH n</code> - Add value to top</div>
          <div><code className="font-mono bg-gray-200 px-1 rounded">POP</code> - Remove top value</div>
          <div><code className="font-mono bg-gray-200 px-1 rounded">DUP</code> - Duplicate top value</div>
          <div><code className="font-mono bg-gray-200 px-1 rounded">SWAP</code> - Swap top two values</div>
        </div>
      </div>
    </div>
  )
}