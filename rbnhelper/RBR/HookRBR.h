#pragma once

void HookOnGameModeChanged();    // Hook RBR gameMode changed handler. Each time a g_pRBRGameMode->gameMode
void HookOnFrame();       // Hook RBR "frame handler". The OnFrame is called by RBR only when countdown begins in racing and replay modes, not while RBR is in menu screens
void HookOnBeginScene();  // Hook RBR "DX9 BeginScene" handler
void HookOnEndScene();    // Hook RBR "DX9 EndScene" handler
void HookOnStartRace();   // Hook on RBR "Starting a custom stage as a quick race" event
