import { invoke } from '@tauri-apps/api/core'
import { useRef, useState } from 'react'

type ChatMessage = {
  text: string
  role: 'USER' | 'ASSISTANT'
  time: number
}

const ChatContainer = () => {
  const [isLoading, setLoading] = useState(false)
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const inputEl = useRef<HTMLInputElement>(null)

  const handleSend = () => {
    const text = inputEl.current?.value || ''
    if (!text) {
      return
    }

    inputEl.current!.value = ''

    setLoading(true)
    setMessages(prev => [...prev, { text, role: 'USER', time: Date.now() }])

    invoke('api_request', {
      contents: [
        {
          parts: [
            {
              text
            }
          ]
        }
      ]
    })
      .then(result => {
        const { candidates = [] } = result as any
        setMessages(prev => [
          ...prev,
          ...candidates.map((item: any) => ({
            text: item.content.parts.map(({ text }: any) => text).join('\n'),
            role: 'ASSISTANT',
            time: Date.now()
          }))
        ])
      })
      .catch(e => console.log(e))
      .finally(() => setLoading(false))
  }

  return (
    <section className="max-h-full flex flex-col h-screen">
      <div className="grow overflow-y-auto p-4 space-y-4 bg-base-200">
        {messages.map((message, index) => (
          <div
            className={`chat ${message.role === 'USER' ? 'chat-end' : 'chat-start'}`}
            key={`message-${index}`}
          >
            <div className="chat-bubble">{message.text}</div>
          </div>
        ))}
      </div>

      <form
        onSubmit={e => {
          e.preventDefault()
          handleSend()
        }}
      >
        <div className="flex-none p-4 border-base-300 flex items-center gap-4">
          <input
            className="w-full bg-base-100 border border-base-300 rounded-field p-2"
            readOnly={isLoading}
            ref={inputEl}
          />
          <button
            className="btn btn-primary"
            onClick={handleSend}
            disabled={isLoading}
          >
            {isLoading ? (
              <span className="loading loading-bars  loading-md" />
            ) : (
              '发送'
            )}
          </button>
        </div>
      </form>
    </section>
  )
}

export default ChatContainer
