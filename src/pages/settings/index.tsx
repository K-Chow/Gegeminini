import { useEffect, useState, type ChangeEvent } from 'react'
import { invoke } from '@tauri-apps/api/core'

type ApiConfig = {
  app: string
  apiKey: string
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

  const handleModleChange = (
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
        setModels(models)
      })
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
          <input type="radio" name="my-accordion-2" defaultChecked />
          <div className="collapse-title font-semibold">{config.app}</div>
          <div className="collapse-content text-sm">
            <div className="join rounded-field mb-2 w-full">
              <button className="btn join-item ">API key</button>
              <input
                className="input join-item w-80"
                onChange={e => handleApiKeyChange(config.app, e.target.value)}
                value={config.apiKey}
              />
            </div>
            <div className="join rounded-field mb-2 w-full">
              <button className="btn join-item">Model</button>
              <select
                defaultValue=""
                className="select join-item w-80"
                onChange={e => handleModleChange(e, config.app)}
              >
                {models.map(model => (
                  <option key={model.name} value={model.name}>
                    {model.displayName}
                  </option>
                ))}
              </select>
            </div>

            <button
              className="btn btn-success"
              onClick={() => handleSaveApiKey(config.app)}
            >
              保存
            </button>
          </div>
        </div>
      ))}
    </section>
  )
}

export default Settings
