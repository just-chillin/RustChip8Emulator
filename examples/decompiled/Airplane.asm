; Setting used by the chipper assembler
option schip11
option binary
align off

; Recursive Disassembly
End of file
0x0200	ld va, #00
0x0202	ld vb, #04
0x0204	ld vc, #01
0x0206	ld vd, #00
0x0208	ld ve, #02
loc_020a:   ; == START OF CODE BLOCK ==
0x020a	call loc_0326
0x020c	call loc_0320
0x020e	ld v0, #30
0x0210	ld v1, #01
0x0212	ld dt, v0
loc_0214:   ; == START OF CODE BLOCK ==
0x0214	ld v0, dt
0x0216	ld st, v1
0x0218	se v0, #00
0x021a	jp loc_0214
0x021c	call loc_0242
loc_021e:   ; == START OF CODE BLOCK ==
0x021e	call loc_0320
0x0220	add vd, #01
0x0222	call loc_0320
0x0224	ld v0, #08
0x0226	sknp v0
0x0228	call loc_030a
0x022a	sne va, #00
0x022c	jp loc_023e
0x022e	ld I, #0362
0x0230	drw v8, v9, #1
0x0232	add v9, #01
0x0234	drw v8, v9, #1
0x0236	sne vf, #01
0x0238	jp loc_02f4
0x023a	sne v9, #18
0x023c	jp loc_02e4
loc_023e:   ; == START OF CODE BLOCK ==
0x023e	call loc_02b2
0x0240	jp loc_021e
loc_0242:   ; == START OF CODE BLOCK ==
0x0242	sne vc, #01
0x0244	call loc_026c
0x0246	sne vc, #02
0x0248	call loc_027a
0x024a	sne vc, #03
0x024c	call loc_0288
0x024e	sne vc, #04
0x0250	call loc_0296
0x0252	sne vc, #05
0x0254	call loc_02a4
0x0256	ld I, #0359
0x0258	drw v6, v7, #2
0x025a	sne v4, #00
0x025c	ret
0x025e	ld I, #0357
0x0260	drw v4, v5, #2
0x0262	sne v2, #00
0x0264	ret
0x0266	ld I, #035b
0x0268	drw v2, v3, #2
0x026a	ret
loc_026c:   ; == START OF CODE BLOCK ==
0x026c	ld v6, #28
0x026e	ld v7, #09
0x0270	ld v4, #00
0x0272	ld v5, #00
0x0274	ld v2, #00
0x0276	ld v3, #00
0x0278	ret
loc_027a:   ; == START OF CODE BLOCK ==
0x027a	ld v6, #28
0x027c	ld v7, #0e
0x027e	ld v4, #28
0x0280	ld v5, #14
0x0282	ld v2, #00
0x0284	ld v3, #00
0x0286	ret
loc_0288:   ; == START OF CODE BLOCK ==
0x0288	ld v6, #28
0x028a	ld v7, #07
0x028c	ld v4, #28
0x028e	ld v5, #0c
0x0290	ld v2, #16
0x0292	ld v3, #11
0x0294	ret
loc_0296:   ; == START OF CODE BLOCK ==
0x0296	ld v6, #28
0x0298	ld v7, #07
0x029a	ld v4, #28
0x029c	ld v5, #0e
0x029e	ld v2, #16
0x02a0	ld v3, #14
0x02a2	ret
loc_02a4:   ; == START OF CODE BLOCK ==
0x02a4	ld v6, #28
0x02a6	ld v7, #05
0x02a8	ld v4, #28
0x02aa	ld v5, #10
0x02ac	ld v2, #16
0x02ae	ld v3, #0b
0x02b0	ret
loc_02b2:   ; == START OF CODE BLOCK ==
0x02b2	ld I, #0359
0x02b4	drw v6, v7, #2
0x02b6	add v6, #fe
0x02b8	drw v6, v7, #2
0x02ba	sne v4, #00
0x02bc	ret
0x02be	ld I, #0357
0x02c0	drw v4, v5, #2
0x02c2	add v4, #02
0x02c4	sne v4, #44
0x02c6	add v4, #c0
0x02c8	drw v4, v5, #2
0x02ca	sne v2, #00
0x02cc	ret
0x02ce	ld I, #035b
0x02d0	drw v2, v3, #2
0x02d2	add v2, #02
0x02d4	sne vc, #04
0x02d6	add v2, #02
0x02d8	sne vc, #05
0x02da	add v2, #02
0x02dc	sne v2, #44
0x02de	add v2, #c0
0x02e0	drw v2, v3, #2
0x02e2	ret
loc_02e4:   ; == START OF CODE BLOCK ==
0x02e4	add vc, #01
0x02e6	ld vd, #00
0x02e8	ld ve, #02
0x02ea	cls
0x02ec	sne vc, #06
0x02ee	ld vc, #01
0x02f0	ld va, #00
0x02f2	jp loc_020a
loc_02f4:   ; == START OF CODE BLOCK ==
0x02f4	ld v0, #06
0x02f6	ld st, v0
0x02f8	add vb, #ff
0x02fa	sne vb, #00
0x02fc	jp loc_0308
0x02fe	ld vd, #00
0x0300	ld ve, #02
0x0302	cls
0x0304	ld va, #00
0x0306	jp loc_020a
loc_0308:   ; == START OF CODE BLOCK ==
0x0308	jp loc_0308
loc_030a:   ; == START OF CODE BLOCK ==
0x030a	sne va, #01
0x030c	ret
0x030e	ld v0, #02
0x0310	ld st, v0
0x0312	ld va, #01
0x0314	ld v8, vd
0x0316	add v8, #01
0x0318	ld v9, ve
0x031a	add v9, #01
0x031c	drw v8, v9, #1
0x031e	ret
loc_0320:   ; == START OF CODE BLOCK ==
0x0320	ld I, #0354
0x0322	drw vd, ve, #2
0x0324	ret
loc_0326:   ; == START OF CODE BLOCK ==
0x0326	ld v4, #19
0x0328	ld v3, #00
0x032a	ld I, #0356
loc_032c:   ; == START OF CODE BLOCK ==
0x032c	drw v3, v4, #1
0x032e	add v3, #08
0x0330	se v3, #40
0x0332	jp loc_032c
0x0334	ld v3, #1e
0x0336	ld v4, #1b
0x0338	ld f, vc
0x033a	drw v3, v4, #5
0x033c	sne vb, #04
0x033e	ld I, #035f
0x0340	sne vb, #03
0x0342	ld I, #0360
0x0344	sne vb, #02
0x0346	ld I, #0361
0x0348	sne vb, #01
0x034a	ld I, #0362
0x034c	ld v3, #01
0x034e	add v4, #02
0x0350	drw v3, v4, #1
0x0352	ret
0x0354	db #80	;GRAPHIC = #       
0x0355	db #f8	;GRAPHIC = #####   
0x0356	db #ff	;GRAPHIC = ########
0x0357	db #80	;GRAPHIC = #       
0x0358	db #e0	;GRAPHIC = ###     
0x0359	db #10	;GRAPHIC =    #    
0x035a	db #70	;GRAPHIC =  ###    	ASCII(p)
0x035b	db #88	;GRAPHIC = #   #   
0x035c	db #ee	;GRAPHIC = ### ### 
0x035d	db #11	;GRAPHIC =    #   #
0x035e	db #77	;GRAPHIC =  ### ###	ASCII(w)
0x035f	db #aa	;GRAPHIC = # # # # 
0x0360	db #a8	;GRAPHIC = # # #   
0x0361	db #a0	;GRAPHIC = # #     
0x0362	db #80	;GRAPHIC = #       
0x0363	db #00	;GRAPHIC =         
