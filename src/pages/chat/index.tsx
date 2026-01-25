const ChatContainer = () => {
  return (
    <section className="max-h-full flex flex-col h-screen">
      <div className="grow overflow-y-auto p-4 space-y-4 bg-base-200"></div>
      <div className="flex-none p-4 border-base-300 flex items-center gap-4">
        <input className="w-full bg-base-100 border border-base-300 rounded-field p-2" />
        <button className="btn btn-primary">Send</button>
      </div>
    </section>
  )
}

export default ChatContainer
