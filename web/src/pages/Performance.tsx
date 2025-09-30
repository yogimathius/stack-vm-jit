import React from 'react'
import { BarChart3 } from 'lucide-react'

export default function Performance() {
  return (
    <div className="text-center py-12">
      <BarChart3 className="w-16 h-16 text-gray-400 mx-auto mb-4" />
      <h1 className="text-2xl font-bold text-gray-900 mb-2">Performance Benchmarks</h1>
      <p className="text-gray-600">
        Detailed performance analysis and benchmarking tools coming soon
      </p>
    </div>
  )
}