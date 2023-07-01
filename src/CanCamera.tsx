import { useEffect, useState } from 'react'
import { observer } from 'mobx-react-lite'
import { ObservablePromise } from 'mobx-observable-promise'
import VideoContext from './VideoContext'
import CameraSuccess from './CameraSuccess'
import VideoPromise from './VideoPromise'

const CanCamera = observer(() => {
  const [observablePromise] = useState(() => {
    const observablePromise: VideoPromise = new ObservablePromise(async deviceId => {
      const mediaStream = await navigator.mediaDevices.getUserMedia({
        audio: false,
        video: {
          ...deviceId !== undefined
            ? { deviceId }
            : {
                facingMode: 'user'
              }
          // frameRate: 60
        }
      })
      return mediaStream
    })
    const selectedDeviceId = localStorage.getItem('cameraDeviceId') ?? undefined
    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    observablePromise.execute(selectedDeviceId).catch()
    // eslint-disable-next-line @typescript-eslint/return-await
    return observablePromise
  })

  useEffect(() => {
    if (observablePromise.error !== null) {
      console.error(observablePromise.error)
    }
  }, [observablePromise.error])

  return (
    <>
      {observablePromise.wasSuccessful
        ? (
          <VideoContext.Provider value={observablePromise}>
            <CameraSuccess />
          </VideoContext.Provider>)
        : observablePromise.isError
          ? 'Error getting camera. Make sure it is not being used by another app.'
          : 'Getting camera'}
    </>
  )
})

export default CanCamera
