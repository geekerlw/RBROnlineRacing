#include "IRust.h"

PluginRust *g_pRBRPlugin = nullptr;

#ifdef __cplusplus
extern "C" {
#endif

void* RBR_InitPlugin(void* arg) {
    IRBRGame *pGame = static_cast<IRBRGame*>(arg);
    if (g_pRBRPlugin == nullptr)
    {
        g_pRBRPlugin = new PluginRust(pGame);
    }

    return (void*)g_pRBRPlugin;
}

void RBR_SetInitialize(PlugInitialize func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fPlugInitialize = func;
    }
}

void RBR_SetDrawFrontEndPage(PlugDrawFrontEndPage func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fDrawFrontEndPage = func;
    }
}

void RBR_SetDrawResultUI(PlugDrawResultUI func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fDrawResultUI = func;
    }
}

void RBR_SetHandleFrontEndEvents(PlugHandleFrontEndEvents func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fHandleFrondendEvents = func;
    }
}

void RBR_SetTickFrontEndPage(PlugTickFrontEndPage func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fTickFrontEndPage = func;
    }
}

void RBR_SetStageStarted(PlugStageStarted func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fStageStarted = func;
    }
}

void RBR_SetHandleResults(PlugHandleResults func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fHandleResults = func;
    }
}

void RBR_SetCheckPoint(PlugCheckPoint func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fCheckPoint = func;
    }
}

void RBR_SetOnGameModeChanged(PlugOnGameModeChanged func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fOnGameModeChanged = func;
    }
}

void RBR_SetOnFrame(PlugOnFrame func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fOnFrame = func;
    }
}

void RBR_SetOnStageStarted(PlugOnFrame func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fOnStartRace = func;
    }
}

void RBR_SetOnBeginScene(PlugOnBeginScene func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fOnBeginScene = func;
    }
}

void RBR_SetOnEndScene(PlugOnEndScene func) {
    if (g_pRBRPlugin) {
        g_pRBRPlugin->m_fOnEndScene = func;
    }
}

#ifdef __cplusplus
}
#endif