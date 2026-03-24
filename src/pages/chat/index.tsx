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
import { Virtuoso } from 'react-virtuoso'
import type { List } from '@/types'

type ChatMessage = {
  content: string
  role: 'USER' | 'MODEL'
  time: number
}

const ChatContainer = () => {
  const [currentPage, setCurrentPage] = useState(1)
  const [firstItemIndex, setFirstItemIndex] = useState(0)
  const [isLoading, setLoading] = useState(false)
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const inputEl = useRef<HTMLInputElement>(null)
  const { config } = useGlobalContext()

  const theme = useMemo(
    () => (config.theme === 'light' ? materialLight : materialDark),
    [config.theme]
  )

  const getMessages = () => {
    invoke('response_messages', { page: { number: currentPage, size: 10 } })
      .then(result => {
        console.log(result)
        const { items = [], page, size, total } = result as List<ChatMessage>
        if (size * page < total) {
          setCurrentPage(page + 1)
        }
        setFirstItemIndex(total - page * size)

        setMessages([...items, ...messages])
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
      text
    })
      .then(result => {
        console.log(result)
        setMessages(prev => [...prev, ...(result as any)])
      })
      .catch(e => console.log(e))
      .finally(() => setLoading(false))
  }

  useEffect(() => {
    getMessages()
  }, [])

  return (
    <section className="max-h-full flex flex-col h-screen max-w-full">
      <div className="grow overflow-hidden bg-base-200 max-w-full">
        <Virtuoso
          data={messages}
          firstItemIndex={firstItemIndex}
          followOutput="smooth"
          startReached={() => {
            getMessages()
          }}
          components={{
            Header: () => (
              <div className="h-20 p-4 text-base-content/30 text-center">
                --.--
              </div>
            ),
            Footer: () => <div className="h-24" />
          }}
          itemContent={(index, message: ChatMessage) => (
            <div
              className={`chat max-w-full overflow-hidden my-4 px-4 ${message.role === 'USER' ? 'chat-end ' : 'chat-start'}`}
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
                        const { children, className, ...rest } = props
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
          )}
        />
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
