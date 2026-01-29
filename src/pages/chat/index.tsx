import { invoke } from '@tauri-apps/api/core'
import { useState, type ChangeEvent } from 'react'

const ChatContainer = () => {
  const [text, setText] = useState('')

  const handleInput = async (e: ChangeEvent<HTMLInputElement>) => {
    await setText(e.currentTarget.value)
  }

  const handleSend = async () => {
    try {
      const result = await invoke('api_request', {
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
      console.log(result)
    } catch (e) {
      console.log(e)
    }
  }
  return (
    <section className="max-h-full flex flex-col h-screen">
      <div className="grow overflow-y-auto p-4 space-y-4 bg-base-200"></div>
      <div className="flex-none p-4 border-base-300 flex items-center gap-4">
        <input
          className="w-full bg-base-100 border border-base-300 rounded-field p-2"
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
