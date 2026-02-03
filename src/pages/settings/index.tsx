import { useState, type ChangeEvent } from 'react'
import { invoke } from '@tauri-apps/api/core'

const Settings = () => {
  const [apiKey, setApikey] = useState('')
  const handleApiKeyChange = (e: ChangeEvent<HTMLInputElement>) => {
    setApikey(e.target.value)
  }

  const handleSaveApiKey = () => {
    invoke('save_api_config', {
      configs: [{ app: 'gemini', api_key: apiKey }]
    })
      .then(result => console.log(result))
      .catch(err => console.log(err))
  }

  return (
    <section className="p-4">
      <h2 className="mb-8 text-2xl">设置</h2>
      <div className="collapse collapse-arrow bg-base-100 border border-base-300">
        <input type="radio" name="my-accordion-2" defaultChecked />
        <div className="collapse-title font-semibold">Gemini</div>
        <div className="collapse-content text-sm">
          <div className="join rounded-field">
            <button className="btn join-item ">API key</button>
            <input className="input join-item" onChange={handleApiKeyChange} />
            <button
              className="btn btn-success join-item"
              onClick={handleSaveApiKey}
            >
              保存
            </button>
          </div>
        </div>
      </div>
    </section>
  )
}

export default Settings
