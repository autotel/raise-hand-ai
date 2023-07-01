import { stayStillTime } from '../../config'
import DotPlacerData from '../../dotPlacer/Data'
import cleanupDotPlacer from '../../dotPlacer/cleanup'
import drawWithPose from '../../dotPlacer/drawWithPose/drawWithPose'
import tickDotPlacer from '../../dotPlacer/tick/tick'
import YesNoData from '../../handYesNo/Data'
import HandYesNo from '../../handYesNo/HandYesNo'
import cleanupYesNo from '../../handYesNo/cleanup'
import ProgressBar from '../../handYesNo/progressBar/ProgressBar'
import tickHandYesNo from '../../handYesNo/tick'
import YesSound from '../../handYesNo/yesSound/YesSound'
import Side from '../../raiseHandProgress/Side'
import sideNames from '../../sideNames'
import Heading from '../Heading'
import SceneFns from '../SceneFns'

const calibrateBottomCornerFns: SceneFns<{
  side: Side
  yesNo: YesNoData
  dotPlacer: DotPlacerData
}> = {
  tick: ({ data, pose, ctx, unscaledSize }) => {
    if (pose !== undefined) {
      drawWithPose({
        ctx,
        pose,
        side: data.side
      })
      const newYesNoData = tickHandYesNo(data.yesNo, pose)
      const newDotPlacerData = newYesNoData.raised
        ? cleanupDotPlacer(data.dotPlacer)
        : tickDotPlacer({
          data: data.dotPlacer,
          pose,
          boundaryRect: {
            pos1: {
              x: 0,
              y: 0
            },
            pos2: {
              x: unscaledSize.width,
              y: unscaledSize.height
            }
          }
        })

      return {
        ...data,
        yesNo: newYesNoData,
        dotPlacer: newDotPlacerData
      }
    }
    return data
  },
  render: data => (
    <Heading>
      <h1>
        {data.dotPlacer.earliestPositionEntry !== undefined && (Date.now() - data.dotPlacer.earliestPositionEntry.startTime) / stayStillTime > 0.25
          ? (
            <>
              Keep your hand in place
              <YesSound frequency='G4' />
            </>)
          : (
            <>Move ur {sideNames.get(data.side)} hand to the
              bottom {sideNames.get(1 - data.side)} corner
            </>)}
      </h1>
      {data.dotPlacer.earliestPositionEntry !== undefined && (
        <>
          <ProgressBar
            startTime={data.dotPlacer.earliestPositionEntry.startTime}
            totalTime={stayStillTime}
            style={{
              backgroundColor: 'yellow'
            }}
          />
        </>)}
      <HandYesNo
        data={data.yesNo}
        yesNode={undefined}
        noNode={undefined}
        yesFrequency={NaN}
      />
    </Heading>
  ),
  cleanup: ({ dotPlacer, yesNo }) => {
    cleanupYesNo(yesNo)
    cleanupDotPlacer(dotPlacer)
  }
}

export default calibrateBottomCornerFns
