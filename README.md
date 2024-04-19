# RBROnlineRacing
Richard Burns Rally (RSF) realtime online racing support。

## How to Install
Download RBNHelper_xxx.zip file and unzip into your Richard Burns Rally game root folder.

## How to Unintall
Only need to remove the rbnhelper.dll in the game `Plugins` folder.
Or another simple way is disable the plugin by Rsf launcher.

## Features
- Fully automatic workflow, racer just need to select car,setup and play game.
- Automatically select stages,weather with random way and start the game at three-minute intervals
- Voice notice race join, leave, prepare, load and start etc.
- Start Countdown notice when waiting next race and race prepare time.
- Rank with score record, players has at least 3 score but -5 if you retired.
- Online Ghost Car support, but only one ghost with your previous player.
- Exclude some maps, such as mazes, bad roads, and too long maps.

## Custom configures
You can Find an `RBNHelper.ini` file under `Plugins\RBNHelper` folder, edit it and *restart game*.
I provide the following configurations:
- function switchers in the `Setting` section.
- positions of online leaderboard and progressbar, x, y is offset from the top-left position.
- leaderboard and progressbar's color styles.

## How to Race with others
1. Open game and normally Login to RSF main menu
2. You can find some notice in the top center of screen, such as how many players online and time duration to next race.
3. Join race by Enter `Hotlap` or `Practice` anytime, if race in started, you will auto joined next race, just stay at this menu page.
4. Once next race begined, there are 30 seconds to prepare, game will auto start when time occurs.
5. Once all players loaded, server will pull the handbrake at the same time, 5 seconds countdown to begin driving.
6. Be careful to your driving, you can see other player's progress on the screen left and an brief leaderboard on the top-left screen.
7. Ok, finally we finished race, if you are first, please wait at the result page, once all players finished, a leaderboard with score change will show over the result page.
8. Go back to `Hotlap` or `Practice` menu, and waiting next race begin.