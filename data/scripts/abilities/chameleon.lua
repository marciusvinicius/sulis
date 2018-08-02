function on_activate(parent, ability)
  effect = parent:create_effect(ability:name(), ability:duration())
  effect:add_num_bonus("concealment", 50)

  anim = parent:create_color_anim()
  anim:set_color(anim:param(1.0),
                 anim:param(1.0),
                 anim:param(1.0),
                 anim:param(0.4))
  effect:add_color_anim(anim)
  effect:apply()

  ability:activate(parent)
end