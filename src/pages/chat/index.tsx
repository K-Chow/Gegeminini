import { invoke } from '@tauri-apps/api/core'
import { useEffect, useMemo, useRef, useState } from 'react'
import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter'
import {
  materialDark,
  materialLight
} from 'react-syntax-highlighter/dist/esm/styles/prism'
import { useGlobalContext } from '@/context/GlobalContext'
import type { List } from '@/types'

type ChatMessage = {
  content: string
  role: 'USER' | 'MODEL'
  time: number
}

const ChatContainer = () => {
  const [page, setPage] = useState(1)
  const [isLoading, setLoading] = useState(false)
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const inputEl = useRef<HTMLInputElement>(null)
  const chatEl = useRef<HTMLDivElement>(null)
  const { config } = useGlobalContext()

  const theme = useMemo(
    () => (config.theme === 'light' ? materialLight : materialDark),
    [config.theme]
  )

  const getMessages = () => {
    invoke('get_messages', { page: 1 })
      .then(result => {
        const resultMessages = (result as List<ChatMessage>).items.map(
          message => {
            const content = JSON.parse(message.content)
            return message.role === 'USER'
              ? {
                  ...message,
                  content: content.contents
                    .map((item: any) =>
                      item.parts.map(({ text }: any) => text).join('\n')
                    )
                    .join('\n')
                }
              : {
                  ...message,
                  content:
                    content.candidates
                      ?.map((item: any) =>
                        item.content.parts
                          .map(({ text }: any) => text)
                          .join('\n')
                      )
                      .join('\n') || ''
                }
          }
        )

        setMessages([...resultMessages, ...messages])
      })
      .catch(e => console.log(e))
  }

  const handleSend = () => {
    const text = inputEl.current?.value || ''
    if (!text) {
      return
    }

    inputEl.current!.value = ''

    setLoading(true)
    setMessages(prev => [
      ...prev,
      { content: text, role: 'USER', time: Date.now() }
    ])

    invoke('send_message', {
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
            content: item.content.parts.map(({ text }: any) => text).join('\n'),
            time: Date.now(),
            role: 'MODEL'
          }))
        ])
      })
      .catch(e => console.log(e))
      .finally(() => setLoading(false))
  }

  useEffect(() => {
    getMessages()
  }, [])

  useEffect(() => {
    if (messages.length && chatEl.current) {
      chatEl.current.scrollTo({
        top: chatEl.current.scrollHeight,
        behavior: 'smooth'
      })
    }
  }, [messages, chatEl])

  return (
    <section className="max-h-full flex flex-col h-screen max-w-full">
      <div
        className="grow overflow-y-auto p-4 space-y-4 bg-base-200 max-w-full"
        ref={chatEl}
      >
        {messages.map((message, index) => (
          <div
            className={`chat max-w-full whitespace-pre-wrap overflow-hidden ${message.role === 'USER' ? 'chat-end ' : 'chat-start'}`}
            key={`message-${index}`}
          >
            <div
              className={`chat-bubble  max-w-[90%] ${message.role === 'USER' ? 'chat-bubble-accent' : ''} `}
            >
              {message.content ? (
                <Markdown
                  children={message.content}
                  remarkPlugins={[remarkGfm]}
                  components={{
                    code(props) {
                      const { children, className, node, ...rest } = props
                      const match = /language-(\w+)/.exec(className || '')
                      return match ? (
                        <SyntaxHighlighter
                          {...rest}
                          PreTag="div"
                          children={String(children).replace(/\n$/, '')}
                          language={match[1]}
                          style={theme}
                        />
                      ) : (
                        <code
                          {...rest}
                          style={{ maxWidth: '100%' }}
                          className={className}
                        >
                          {children}
                        </code>
                      )
                    }
                  }}
                />
              ) : (
                <span className="text-error">遇到了一些问题</span>
              )}
            </div>
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
