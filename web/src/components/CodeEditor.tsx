import React from 'react'
import Editor from '@monaco-editor/react'

interface CodeEditorProps {
  value: string
  onChange: (value: string) => void
  language: string
  currentLine?: number
  height?: string
}

export default function CodeEditor({ 
  value, 
  onChange, 
  language, 
  currentLine,
  height = '300px' 
}: CodeEditorProps) {
  const handleEditorChange = (value: string | undefined) => {
    onChange(value || '')
  }

  const editorOptions = {
    minimap: { enabled: false },
    lineNumbers: 'on' as const,
    scrollBeyondLastLine: false,
    fontFamily: 'JetBrains Mono, Consolas, Monaco, monospace',
    fontSize: 14,
    tabSize: 2,
    insertSpaces: true,
    automaticLayout: true,
    wordWrap: 'on' as const,
    theme: 'vs-light',
    readOnly: false,
  }

  return (
    <div className="code-editor">
      <Editor
        height={height}
        language={language === 'assembly' ? 'plaintext' : language}
        value={value}
        onChange={handleEditorChange}
        options={editorOptions}
        onMount={(editor, monaco) => {
          // Highlight current line if provided
          if (currentLine !== undefined) {
            editor.deltaDecorations([], [{
              range: new monaco.Range(currentLine + 1, 1, currentLine + 1, 1),
              options: {
                isWholeLine: true,
                className: 'current-line-highlight',
                glyphMarginClassName: 'current-line-glyph'
              }
            }])
          }

          // Custom assembly language syntax highlighting
          monaco.languages.register({ id: 'vm-assembly' })
          monaco.languages.setMonarchTokensProvider('vm-assembly', {
            tokenizer: {
              root: [
                [/^[a-zA-Z_][a-zA-Z0-9_]*:/, 'keyword.label'],
                [/\b(PUSH|POP|ADD|SUB|MUL|DIV|MOD|DUP|SWAP|JMP|JT|JF|CALL|RET|HALT|NOP|AND|OR|XOR|NOT|SHL|SHR|EQ|NE|LT|LE|GT|GE|LOAD|STORE|NEW|GET_FIELD|SET_FIELD)\b/, 'keyword'],
                [/;.*$/, 'comment'],
                [/\b\d+\b/, 'number'],
                [/"[^"]*"/, 'string'],
                [/\b[a-zA-Z_][a-zA-Z0-9_]*\b/, 'identifier'],
              ]
            }
          })

          monaco.editor.defineTheme('vm-theme', {
            base: 'vs',
            inherit: true,
            rules: [
              { token: 'keyword', foreground: '0066cc', fontStyle: 'bold' },
              { token: 'keyword.label', foreground: '6f42c1', fontStyle: 'bold' },
              { token: 'comment', foreground: '6a737d', fontStyle: 'italic' },
              { token: 'number', foreground: '005cc5' },
              { token: 'string', foreground: '032f62' },
            ],
            colors: {
              'editor.background': '#ffffff',
              'editor.lineHighlightBackground': '#f6f8fa',
              'editorLineNumber.foreground': '#6a737d',
            }
          })

          monaco.editor.setTheme('vm-theme')
        }}
      />
      
      <style>{`
        .current-line-highlight {
          background-color: rgba(59, 130, 246, 0.1) !important;
          border-left: 3px solid #3b82f6 !important;
        }
        .current-line-glyph {
          background-color: #3b82f6;
          width: 4px !important;
          margin-left: 3px;
        }
      `}</style>
    </div>
  )
}