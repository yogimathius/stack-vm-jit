import React from 'react'
import { BookOpen, Play, Target, Zap } from 'lucide-react'

export default function Tutorial() {
  return (
    <div className="max-w-4xl mx-auto space-y-8">
      <div className="text-center">
        <h1 className="text-3xl font-bold text-gray-900 mb-4">VM Tutorial</h1>
        <p className="text-xl text-gray-600">
          Learn virtual machine concepts and JIT compilation through interactive examples
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-6">
        <div className="bg-white p-6 rounded-lg border hover:shadow-lg transition-shadow">
          <div className="flex items-center gap-3 mb-4">
            <Play className="w-8 h-8 text-blue-600" />
            <h3 className="text-xl font-semibold">Getting Started</h3>
          </div>
          <p className="text-gray-600 mb-4">
            Learn the basics of stack-based virtual machines and how to write simple assembly programs.
          </p>
          <ul className="space-y-2 text-sm text-gray-700">
            <li>• Understanding the stack</li>
            <li>• Basic instructions</li>
            <li>• Your first program</li>
          </ul>
        </div>

        <div className="bg-white p-6 rounded-lg border hover:shadow-lg transition-shadow">
          <div className="flex items-center gap-3 mb-4">
            <Target className="w-8 h-8 text-green-600" />
            <h3 className="text-xl font-semibold">Advanced Concepts</h3>
          </div>
          <p className="text-gray-600 mb-4">
            Explore memory management, function calls, and complex control flow.
          </p>
          <ul className="space-y-2 text-sm text-gray-700">
            <li>• Memory operations</li>
            <li>• Function calls</li>
            <li>• Error handling</li>
          </ul>
        </div>

        <div className="bg-white p-6 rounded-lg border hover:shadow-lg transition-shadow">
          <div className="flex items-center gap-3 mb-4">
            <Zap className="w-8 h-8 text-purple-600" />
            <h3 className="text-xl font-semibold">JIT Compilation</h3>
          </div>
          <p className="text-gray-600 mb-4">
            Understand just-in-time compilation and performance optimization techniques.
          </p>
          <ul className="space-y-2 text-sm text-gray-700">
            <li>• Hot-spot detection</li>
            <li>• Optimization strategies</li>
            <li>• Performance profiling</li>
          </ul>
        </div>

        <div className="bg-white p-6 rounded-lg border hover:shadow-lg transition-shadow">
          <div className="flex items-center gap-3 mb-4">
            <BookOpen className="w-8 h-8 text-orange-600" />
            <h3 className="text-xl font-semibold">Reference</h3>
          </div>
          <p className="text-gray-600 mb-4">
            Complete instruction reference and API documentation.
          </p>
          <ul className="space-y-2 text-sm text-gray-700">
            <li>• Instruction set</li>
            <li>• Memory model</li>
            <li>• Performance tips</li>
          </ul>
        </div>
      </div>
    </div>
  )
}