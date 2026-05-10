import { invoke } from '@tauri-apps/api/core'

const Voice = () => {
  return (
    <div className="max-h-full flex flex-col h-screen max-w-full">
      <div className="p-4">
        <button onClick={() => invoke('trigger_recording')}>
          Start voice recording
        </button>
      </div>
    </div>
  )
}

export default Voice
