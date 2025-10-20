# CHIP-8 Terminal Emulator Journey

## October 14

So I've been wanting to make an emulator for a while now tbh. I was out of ideas for terminalcraft then I finally came up with an idea about making a CHIP-8 emulator and I thought why not? I spent the day reading about it and honestly it looked doable but also kinda scary lol. CHIP-8 is kinda an old computer system from the 70s that people use to learn emulation. It has 35 instructions which for me it is a lot.

I went through Cowgod's CHIP-8 reference like 3 times. The first time I barely understood anything. Second time things started clicking. By the third read I was like okay I can do this. The memory layout is kinda simple enough (barey) like 4KB total, programs start at 0x200, there's a stack for subroutines, 16 registers, and a 64x32 display. This is all I know for now.

I spent time planning how I'll structure everything. I want to make Inspector Mode which I see as the killer feature like you can pause the game and see exactly what the CPU is doing. I think that's gonna be sick for learning how these old games work. It is my unique angle btw

Also decided to use Rust because why not make my life harder lol. But seriously Rust is perfect for this like memory safety, performance, and I've been wanting to get better at it. That's all for today

## October 15

Project setup day! I initialized the Cargo project and added all dependencies. I'm using ratatui for the terminal UI which I used before so that's good. Added crossterm for input handling, rand for the random opcode, clap for CLI args, and serde for save states. I am missing something but anyways

Made the basic file structure:
- main.rs for entry point
- emulator.rs for the core VM
- opcodes.rs for all 35 instructions  
- ui.rs for the TUI
- disassembler.rs for showing assembly

I also sketched out the Emulator struct with all the fields, memory, registers, PC, stack, timers, display buffer, etc. Didn't write much code yet just planning and setup. I want to make sure I understand the architecture before diving in.

I found out about the font sprites that need to be loaded at memory 0x000. It's 80 bytes of data for displaying hex digits 0-F. Each character is 5 bytes. Pretty clever tbh.

## October 16

Okay time to actually code! Started with emulator.rs today. Implemented the Emulator::new() function and loaded the font sprites into memory. Made the load_rom function that copies ROM data starting at 0x200. 

Then I made the fetch() method that reads the next opcode. CHIP-8 opcodes are 2 bytes so you gotta read two bytes and combine them. That was straightforward.

I also implemented the basic timer logic. CHIP-8 has delay and sound timers that count down at 60Hz. I'll need to handle that in the main loop later.

Started thinking about how to structure the opcode execution. It's gonna be a big match statement based on the first nibble of the opcode. Some opcodes need more checking like 0x8XYN has 9 different operations based on the last nibble.

Didn't get to actual opcodes yet but the foundation is there. Tomorrow is opcode day and I'm both excited and scared lol. I lockedin since earliest morning to now evening. 

## October 17

OPCODE MARATHON! I spent like the whole day implementing all 35 CHIP-8 opcodes. This was intense ngl. 

Started with the simple ones:
- 00E0: Clear display (just reset the display buffer to false)
- 1NNN: Jump (set PC to NNN)
- 6XNN: Set register (VX = NN)
- 7XNN: Add to register (VX += NN)

Those were easy (I am lying). Then came the tricky ones. The 8XYN group has all the register operations - OR, AND, XOR, ADD with carry, SUB with borrow, shifts. The shifts were confusing at first because you gotta set VF to the bit that gets shifted out.

The draw sprite opcode DXYN was the hardest. You gotta:
- Get X and Y coordinates from registers
- Read N bytes from memory starting at I
- XOR each pixel onto the display
- Wrap around screen edges  
- Set VF if any pixels got turned off (collision detection)

I probably spent 3 hours just on that one opcode getting it right. Had to draw it out on paper to understand the bit manipulation.

The FX0A opcode (wait for key press) was interesting. You can't just block as you gotta check if a key is pressed and only advance PC when one is. I added a waiting_for_key field to track this.

By the end of the day I had all 35 opcodes implemented. No idea if they work yet lol. That's tomorrow's problem.

## October 18

Today was TUI and fixing bug day! I started working on ui.rs to actually see if my emulator works.

Set up the basic ratatui structure with crossterm backend. Made a render loop that runs at 60 FPS. The CHIP-8 display is 64x32 pixels so I had to map that to terminal characters. I used block characters (█) for ON pixels and spaces for OFF.

The keyboard mapping was fun. CHIP-8 has a hex keypad (0-F) so I mapped it to:
```
1 2 3 4
Q W E R  
A S D F
Z X C V
```

This matches the original CHIP-8 layout pretty well.

Added some CRT effects (love it)! I implemented a ghosting buffer that makes pixels fade out slowly instead of disappearing instantly. It looks so retro and cool ngl. Also added scanlines by alternating character intensity.

Tested with IBM logo ROM and... IT WORKED!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! First try! I was shocked lol. Seeing the IBM logo appear in my terminal was so satisfying.

Then I tested Pong and uhhhh the paddle wasn't moving. Pressed the keys, nothing happened. Oof. Input system is broken. I'll fix that tho. it turned into bug fixing nightmare lol.

First issue: the emulator didn't work on Windows PowerShell! I got this error:
```
error: Error calling dlltool 'dlltool.exe': program not found
```

wtf is dlltool?????????????? Turns out it's a MinGW thing. Rust on Windows with the GNU toolchain needs MinGW-w64 installed. But I didn't want to deal with that so I just used WSL2. Installed Ubuntu, installed Rust inside WSL, and boom it built fine.

Second issue (the big one): GAMES WERE UNPLAYABLE. Like completely broken. I could run them but pressing keys did nothing. The paddle in Pong wouldn't move. Tetris pieces wouldn't rotate. 

I spent hours debugging this. First I thought maybe my keypad mapping was wrong. Nope, keys were being detected. Then I thought maybe opcodes were broken. Nope, everything else worked.

Finally found it lol the input handler was clearing the keypad array EVERY TIME it checked for input! So keys would be true for like 1 millisecond then get reset to false. The game never saw them as pressed.

First fix attempt: removed the line that clears the keypad. This made keys "stick" - once you press a key it stays pressed until you press another key. Better than nothing but still weird.

I showed this to copilot and we realized the issue. Terminals send repeated key events while you hold a key down. So the solution we concluded is:
1. Clear all keys at START of frame
2. Collect ALL key events during that frame  
3. Apply the keys BEFORE running the game logic
4. Game sees keys as held!

Implemented this and FINALLY IT WORKED! Pong is playable! Tetris works! I played Pong for like 10 minutes straight just because I was so happy it worked lol.

## October 19

Inspector Mode day! This is the feature I was most excited about.

I designed a multi-panel layout:
- Top left: Game display (64x32)
- Top right: CPU state (all registers, PC, SP, I, timers)
- Bottom left: Disassembly (next 12 opcodes with mnemonics)
- Bottom right: Controls help

The hardest part was making the disassembler. I had to write code that takes an opcode like 0x6A05 and converts it to "LD VA, 05". Did this for all 35 opcodes. Lots of pattern matching and string formatting.

The single-step feature was easier than I thought. Just execute one cycle, then re-render. Press S to step, press C to continue.

Added checkpoint save/load too. Press K to save current state, L to restore. This is done by cloning the entire emulator state. Simple but effective.

The coolest thing is watching games execute instruction by instruction. You can see exactly how Pong checks for key presses, updates the paddle position, checks for collisions, etc. It's like X-ray vision for retro games.

Tested everything and it all works! Inspector Mode is definitely the best part of this project.

Final polish to fix a bunch of small things:

1. Added command line arguments with clap. You can now do:
   ```
   term-8 roms/pong.ch8 --speed 15
   ```
   to adjust how many cycles per frame.

2. Created a roms/ directory and added 3 public domain ROMs:
   - ibm_logo.ch8 (test ROM)
   - pong.ch8 (the classic)
   - tetris.ch8 (works perfectly!)

3. Wrote initial documentation. Made README.md, INSTALL.md with Windows WSL instructions, and a guide on how to use Inspector Mode.

4. Set up GitHub Actions for automated builds. This will create binaries for Windows, Linux, and macOS so people don't have to deal with the dlltool nonsense.

Also added proper error handling everywhere. Before the program would just panic if you gave it a bad ROM file. Now it shows a nice error message.

Tested on my friend's computer (Windows) and it worked through WSL! Tested on macOS too and it just worked. Linux obviously works since I've been developing on WSL.

The whole thing is like 1000 lines of Rust across 5 files. Pretty clean honestly tho it wasn't something organized until now lmao.

I tried my best to work with copilot to make the README.md comprehensive. It explains what CHIP-8 is, what term-8 does, how Inspector Mode works, includes the full keymap, links to ROM sources, everything. If someone clones the repo they have all the info they need.

Also verified one more time that games work. Played through several Tetris games, played Pong against myself (still lost lmao), ran the IBM logo test. All good.

Built the release binary with cargo build --release. The binary is only 4.5MB which is tiny! Rust optimization is insane.

## My final thought

This was one of my best hackclub projects tbh that's why I documented it. I learned so much:

1. **Emulation is hard but not impossible** - CHIP-8 is beginner-friendly but you still gotta understand CPU architecture, memory, timing, etc.

2. **Debugging is 50% of the work** - The input bug took HOURS to fix. Five different attempts before getting it right. That's just how it is.

3. **Platform compatibility is annoying** - Windows has weird toolchain issues. WSL is a lifesaver. Cross-platform is harder than it looks. 

4. **Documentation matters** - I spent a full day just writing docs and cleaning up and credits to AI for streamlining documentation process. But now anyone can understand and use this project.

5. **Inspector Mode was worth it** - This isn't just another CHIP-8 emulator. Being able to see inside the CPU while games run is genuinely educational and cool.

Would I do it again? Yes! 

Would I choose Windows as my dev environment? Hell no lol, WSL all the way.



I just hope josias make v6 :)


Thanks Hack Club for the motivation to actually finish this!

