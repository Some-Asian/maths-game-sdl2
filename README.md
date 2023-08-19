# maths-game-sdl2

TODO:
- ~~Create this TODO list~~
- ~~add SDL2, SDL2-mixer, SDL2-ttf to the repo~~
- ~~make a screen~~
- import fonts u like, figure out how to print them to the screen
- create the renderer, game, etc structs & enums
- ~~figure out keyboard inputs~~
- figure out the game loop lol
- figure out the difficulty mechanism
- ~~make the question generation algorithm~~
- add more to this todo list (and actually tick stuff off when u do them so u feel rewarded or smth)

## Speed Maths
*A fun, hands-on way to hone the maths skills you need the most*

Think of this project as a mixture of Mathletics Live, and Monkeytype. Through repitition and variation, players will be able to steadily improve upon their fundamental mental maths skills, including (but not limited to):
- Addition, Subtraction, Multiplication and Division
- Fraction and Ratio Simplification
- Exponents and Square Roots
- A combination of everything!

The primary concept is simple: players are presented a question, and they answer it as fast as they can. Various modes will be available:
- Marathon: Last as long as possible, as you answer increasingly difficult questions with a speedy time limit
- Zen: Choose a difficulty, and answer as many questions as you can within 60 seconds
- Sprint: Choose a difficulty, and answer 10/20/50/100 questions as fast as you can

Find a particular type of question you struggle with? Adjust the categories of questions you'll be asked to challenge yourself, or hone your best skills!

All of this, packaged in a neat, minimalistic UI with a versatile theming system!

### Difficulty Parameters
| Digits | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 0 |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|Difficulty|0.03|0.06|0.09|0.09|0.07|0.10|0.13|0.11|0.08|0.01|

Estimated Digit Difficulty = Place * Digit Difficulty
e.g a 3 in the hundreds place would add 0.18 to the question difficulty

Operation Difficulties: Let P, Q = # of digits in n1, n2 of equation respectively, p, q = n1, n2 respectively
| Operation | plus | minus | times | divide | simplify | square | cube | sqrt |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|Difficulty|0.5 + 0.25P + 0.25Q|0.5 + 0.25P + 0.3Q|0.8 + sqrt(0.09p + 0.09q)|0.8 + 0.45P + 0.8(P - Q)|
