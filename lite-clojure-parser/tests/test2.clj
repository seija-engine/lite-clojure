(if-comp dynHasPostEffect
      [posteffect-draw camera-id camera-query depth-texture camera-target]
      [(fc [] (node DrawPassNodeID  camera-id camera-query [camera-target] depth-texture "Foward")) camera-id camera-query depth-texture camera-target]
)