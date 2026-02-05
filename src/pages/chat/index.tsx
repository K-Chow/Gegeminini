import { invoke } from '@tauri-apps/api/core'
import { useRef, useState, type ChangeEvent, type KeyboardEvent } from 'react'
import { useKey } from 'react-use'

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

    setLoading(true)

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
        const { candidates } = result as any
        setMessages(
          messages.concat([{ text, role: 'USER', time: Date.now() }]).concat(
            candidates.map((item: any) => ({
              text: item.content.parts.map(({ text }: any) => text).join('\n'),
              role: 'ASSISTANT',
              time: Date.now()
            }))
          )
        )
        inputEl.current!.value = ''
      })
      .catch(e => console.log(e))
      .finally(() => setLoading(false))
  }

  useKey('Enter', () => handleSend(), {
    event: 'keydown',
    target: window
  })

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
      <div className="flex-none p-4 border-base-300 flex items-center gap-4">
        <input
          className="w-full bg-base-100 border border-base-300 rounded-field p-2"
          disabled={isLoading}
          ref={inputEl}
        />
        <button className="btn btn-primary" onClick={handleSend}>
          Send
        </button>
      </div>
    </section>
  )
}

export default ChatContainer
