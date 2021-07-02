#_(Entity
   Sprite
   Draw
   Mesh
   pipelines
   Material [sprite.material]
   CameraNode ---------------|
   |        (name:mainPass)
   | ----------PassNode ---------- WindowSwap
   |      (foreach pipeline)
   ResNode [sprite.material] --|)

;sprite.material 文件
(declare vert-shader)
(declare frag-shader)
(declare set-position vert.pos in-rect set-color texture-in vert.uv discard-color mk-node link-> window-swap)

{:for-node "mainPass" ;默认为*所有PassNode都会使用 
 :pipes [{:propertys {:color "Color" :texture "Texture"}
          :z-test      true
          :culling-mask   false
          :alpha-blend false
          :vert-shader vert-shader
          :frag-shader frag-shader}
         {;;;可以多个pipeline 会同时绘制到一个目标上
          }]}

;sprite-mask.material
(declare vert-shader)
(declare frag-shader)
{:pipes [{:propertys {:color "Color"
                      :texture "Texture"
                      :mask "Rect"}
          :alpha-blend ["SrcAlpha" "Dst-Alpha"]
          :z-test false}]}

(defn vert-shader [model vp vert color texture mask]
  (set-position (* model vp vert.pos)))

(defn vert-shader [model vp vert color texture mask]
  (if (in-rect mask vert.pos)
    (set-color (* texture-in texture texture vert.uv) color)
    (discard-color)))

;graph-default.graph 文件
(def camera-node (mk-node :CamereNode {:type :2d}))
(def sprite-res-node (mk-node :MaterialResNode {:material "sprite"}))
(def main-pass-node (mk-node :MainPass))

(link-> camera-node     main-pass-node)
(link-> sprite-res-node main-pass-node)
(link-> main-pass-node  window-swap)