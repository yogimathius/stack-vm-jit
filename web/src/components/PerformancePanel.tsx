import React from 'react'
import { PerformanceMetrics, ExecutionStep } from '../types/vm'
import { Clock, Cpu, MemoryStick, Zap } from 'lucide-react'

interface PerformancePanelProps {
  metrics: PerformanceMetrics | null
  executionTrace: ExecutionStep[]
}

export default function PerformancePanel({ metrics, executionTrace }: PerformancePanelProps) {
  if (!metrics) {
    return (
      <div className="text-center py-8">
        <Clock className="w-12 h-12 text-gray-400 mx-auto mb-3" />
        <p className="text-gray-600">Run a program to see performance metrics</p>
      </div>
    )
  }

  const formatNumber = (num: number) => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`
    return num.toFixed(0)
  }

  const formatTime = (ms: number) => {
    if (ms < 1) return `${(ms * 1000).toFixed(0)}µs`
    if (ms < 1000) return `${ms.toFixed(1)}ms`
    return `${(ms / 1000).toFixed(2)}s`
  }

  return (
    <div className="space-y-6">
      <h3 className="text-lg font-semibold text-gray-900">Performance Metrics</h3>
      
      {/* Key Metrics */}
      <div className="grid grid-cols-2 gap-4">
        <div className="metric-card p-4 rounded-lg">
          <div className="flex items-center gap-2 mb-2">
            <Cpu className="w-5 h-5 text-blue-600" />
            <span className="font-medium text-gray-900">Throughput</span>
          </div>
          <div className="text-2xl font-bold text-blue-600">
            {formatNumber(metrics.instructionsPerSecond)}
          </div>
          <div className="text-sm text-gray-600">instructions/sec</div>
        </div>

        <div className="metric-card p-4 rounded-lg">
          <div className="flex items-center gap-2 mb-2">
            <Clock className="w-5 h-5 text-green-600" />
            <span className="font-medium text-gray-900">Execution</span>
          </div>
          <div className="text-2xl font-bold text-green-600">
            {formatTime(metrics.executionTime)}
          </div>
          <div className="text-sm text-gray-600">total time</div>
        </div>

        <div className="metric-card p-4 rounded-lg">
          <div className="flex items-center gap-2 mb-2">
            <MemoryStick className="w-5 h-5 text-purple-600" />
            <span className="font-medium text-gray-900">Memory</span>
          </div>
          <div className="text-2xl font-bold text-purple-600">
            {formatNumber(metrics.memoryUsage)}
          </div>
          <div className="text-sm text-gray-600">bytes used</div>
        </div>

        <div className="metric-card p-4 rounded-lg">
          <div className="flex items-center gap-2 mb-2">
            <Zap className="w-5 h-5 text-orange-600" />
            <span className="font-medium text-gray-900">JIT</span>
          </div>
          <div className="text-2xl font-bold text-orange-600">
            {metrics.jitCompilations}
          </div>
          <div className="text-sm text-gray-600">compilations</div>
        </div>
      </div>

      {/* Detailed Stats */}
      <div className="space-y-3">
        <h4 className="font-medium text-gray-900">Detailed Statistics</h4>
        <div className="bg-gray-50 p-3 rounded-lg space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="text-gray-600">Total Instructions:</span>
            <span className="font-mono">{metrics.totalInstructions.toLocaleString()}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600">Average per Instruction:</span>
            <span className="font-mono">
              {formatTime(metrics.executionTime / metrics.totalInstructions)}
            </span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600">GC Collections:</span>
            <span className="font-mono">{metrics.gcCollections}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600">Memory Efficiency:</span>
            <span className="font-mono">
              {metrics.memoryUsage > 0 
                ? `${(metrics.totalInstructions / metrics.memoryUsage * 1000).toFixed(1)} inst/KB`
                : 'N/A'
              }
            </span>
          </div>
        </div>
      </div>
    </div>
  )
}