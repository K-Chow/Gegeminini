import { invoke } from '@tauri-apps/api/core'
import { useState, type ChangeEvent } from 'react'

type ChatMessage = {
  text: string
  role: 'USER' | 'ASSISTANT'
  time: number
}

const ChatContainer = () => {
  const [text, setText] = useState('')
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const handleInput = async (e: ChangeEvent<HTMLInputElement>) => {
    await setText(e.currentTarget.value)
  }

  const handleSend = () => {
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
      })
      .catch(e => console.log(e))
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
      <div className="flex-none p-4 border-base-300 flex items-center gap-4">
        <input
          className="w-full bg-base-100 border border-base-300 rounded-field p-2"
          value={text}
          onChange={handleInput}
        />
        <button className="btn btn-primary" onClick={handleSend}>
          Send
        </button>
      </div>
    </section>
  )
}

export default ChatContainer
