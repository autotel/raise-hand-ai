import { Pose } from '@tensorflow-models/pose-detection'
import Data from '../Data'
import Position from '../Position'

interface Input {
  data: Data
  pose: Pose
  boundaryRect: {
    pos1: Position
    pos2: Position
  }
}

export default Input
