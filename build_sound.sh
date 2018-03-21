ffmpeg -y -i sounds_src/opengameart_ecrivain_sfx_echo_1_2.wav          -ac 2 assets/sounds/all_killed.ogg
ffmpeg -y -i sounds_src/opengameart_ecrivain_sfx_echo_1_1.wav          -ac 2 assets/sounds/portal.ogg
ffmpeg -y -i sounds_src/opengameart_ecrivain_sfx_echo_1_1.wav          -ac 2 assets/sounds/death.ogg
ffmpeg -y -i sounds_src/lmms_clean_rock_bd_modified.wav                -ac 2 assets/sounds/shoot.ogg
ffmpeg -y -i sounds_src/lmms_effects_sonar_modified.wav                -ac 2 assets/sounds/kill.ogg
ffmpeg -y -i sounds_src/lmms_entre.wav                                 -ac 2 assets/sounds/depth_ball_attack.ogg
ffmpeg -y -i sounds_src/lmms_tone4_modified.wav                        -ac 2 assets/sounds/depth_ball_birth_death.ogg
ffmpeg -y -i sounds_src/lmms_fisa_kick.wav                             -ac 2 assets/sounds/bounce.ogg
ffmpeg -y -i sounds_src/opengameart_ecrivain_sfx_echo_1_1_modified.wav -ac 2 assets/sounds/eraser.wav
ffmpeg -y -i sounds_src/lmms_tone15.wav -filter:a "volume=0.2"         -ac 2 assets/sounds/attracted.ogg
lmms -o assets/sounds/mm.ogg -f ogg sounds_src/mm.mmpz
