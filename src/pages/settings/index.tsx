import { useEffect, useState, type ChangeEvent } from 'react'
import { invoke } from '@tauri-apps/api/core'

type ApiConfig = {
  app: string
  apiKey: string
  model: string
}

type ModelItem = {
  displayName: string
  name: string
}

const Settings = () => {
  const [apiConfigs, setApiConfigs] = useState<ApiConfig[]>([])
  const [models, setModels] = useState<ModelItem[]>([])
  const handleApiKeyChange = (app: string, value: string) => {
    setApiConfigs(prev =>
      prev.map(config =>
        config.app === app ? { ...config, apiKey: value } : config
      )
    )
  }

  const getSysConfig = () => {
    invoke('get_api_config')
      .then(res => {
        console.log(res)
        setApiConfigs(res as ApiConfig[])
      })
      .catch(err => console.log(err))
  }

  const handleSaveApiKey = (app: string) => {
    const currentConfig = apiConfigs.find(config => config.app === app)
    invoke('save_api_config', {
      configs: [currentConfig]
    })
      .then(result => console.log(result))
      .catch(err => console.log(err))
  }

  const handleModelChange = (
    e: ChangeEvent<HTMLSelectElement>,
    app: string
  ) => {
    const model = e.target.value
    setApiConfigs(prev =>
      prev.map(config => (config.app === app ? { ...config, model } : config))
    )
  }

  const getModels = () => {
    invoke('get_model_list')
      .then(res => {
        const { models = [] } = res as { models: ModelItem[] }
        console.log(models)
        setModels(models)
      })
      .catch(err => console.log(err))
  }

  const handleClearData = () => {
    invoke('delete_data')
      .then(res => console.log(res))
      .catch(err => console.log(err))
  }

  useEffect(() => {
    getSysConfig()
    getModels()
  }, [])

  return (
    <section className="p-4">
      <h2 className="mb-8 text-2xl">设置</h2>
      {apiConfigs.map(config => (
        <div
          className="collapse collapse-arrow bg-base-100 border border-base-300 mb-2"
          key={`config-${config.app}`}
        >
          <input type="radio" name={`accordion-${config.app}`} defaultChecked />
          <div className="collapse-title font-semibold">{config.app}</div>
          <div className="collapse-content text-sm">
            <div className="join rounded-field mb-2 w-full">
              <button className="btn join-item w-28">API key</button>
              <input
                className="input join-item w-80"
                onChange={e => handleApiKeyChange(config.app, e.target.value)}
                value={config.apiKey}
              />
            </div>
            <div className="join rounded-field mb-2 w-full">
              <button className="btn join-item w-28">Model</button>
              <select
                value={config.model}
                className="select join-item w-80"
                onChange={e => handleModelChange(e, config.app)}
                disabled={models.length === 0 || !config.apiKey}
              >
                {models.map(model => (
                  <option key={model.name} value={model.name}>
                    {model.displayName}
                  </option>
                ))}
              </select>
            </div>

            <div className="flex justify-end">
              <button
                className="btn btn-success mt-4 w-28"
                onClick={() => handleSaveApiKey(config.app)}
              >
                保存
              </button>
            </div>
          </div>
        </div>
      ))}
      <div className="p-4 rounded-box border border-base-300 ">
        <div className="flex justify-end">
          <button
            className="btn btn-error w-28"
            onClick={() => handleClearData()}
          >
            清除数据
          </button>
        </div>
      </div>
    </section>
  )
}

export default Settings
