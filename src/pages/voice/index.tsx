import { invoke } from '@tauri-apps/api/core'
import { useState } from 'react'

const Voice = () => {
  const [isRecording, setIsRecording] = useState(false)
  const handleStartRecording = async () => {
    invoke('start_recording')
      .then(res => {
        console.log(res)
      })
      .catch(err => {
        console.error('Failed to start recording:', err)
      })
  }

  return (
    <div className="max-h-full flex flex-col h-screen max-w-full">
      <div className="p-4">
        <button onClick={handleStartRecording}>
          {isRecording ? 'Stop voice recording' : 'Start voice recording'}
        </button>
      </div>
    </div>
  )
}

export default Voice
