import React from 'react'
import { FileText } from 'lucide-react'

export default function Documentation() {
  return (
    <div className="text-center py-12">
      <FileText className="w-16 h-16 text-gray-400 mx-auto mb-4" />
      <h1 className="text-2xl font-bold text-gray-900 mb-2">Documentation</h1>
      <p className="text-gray-600">
        Comprehensive API documentation and guides coming soon
      </p>
    </div>
  )
}