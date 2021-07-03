(declare set-position vert.pos in-rect set-color texture-in vert.uv discard-color mk-node link-> window-swap)




(defn vert-shader  [^"mode" model vp vert color texture mask]
  (set-position (* model vp vert.pos)))

(defn frag-shader [model vp vert color texture mask]
  (if (in-rect mask vert.pos)
    (set-color (* texture-in texture texture vert.uv) color)
    (discard-color)))


{:pipes [{:propertys {:color "Color"
                      :texture "Texture"
                      :mask "Rect"}
          :alpha-blend ["SrcAlpha" "DstAlpha"]
          :vert-shader vert-shader
          :frag-shader frag-shader
          :z-test false}]}