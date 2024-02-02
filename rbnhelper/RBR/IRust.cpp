#include <windows.h>
#include <string>
#include "IPlugin.h"
#include "IRBRGame.h"

extern "C" {
    typedef void (*PlugInitialize)(void);
    typedef void (*PlugDrawFrontEndPage)(void);
    typedef void (*PlugDrawResultUI)(void);
    typedef void (*PlugHandleFrontEndEvents)(char txtKeyboard, bool bUp, bool bDown, bool bLeft, bool bRight, bool bSelect);
    typedef void (*PlugTickFrontEndPage)(float fTimeDelta);
    typedef void (*PlugStageStarted)(int iMap, std::string ptxtPlayerName, bool bWasFalseStart);
    typedef void (*PlugHandleResults)(float fCheckPoint1, float fCheckPoint2, float fFinishTime, std::string ptxtPlayerName);
    typedef void (*PlugCheckPoint)(float fCheckPointTime, int iCheckPointID, std::string ptxtPlayerName);
}

class PluginRust : public IPlugin {
public:
    PluginRust(IRBRGame* pGame)
        : m_fPlugInitialize(NULL), m_fDrawFrontEndPage(NULL), m_fDrawResultUI(NULL)
        , m_fHandleFrondendEvents(NULL), m_fTickFrontEndPage(NULL), m_fStageStarted(NULL)
        , m_fHandleResults(NULL), m_fCheckPoint(NULL)
        , m_pGame(pGame) {};
    virtual ~PluginRust() {};

public:
    const char* GetName(void) {
        static bool initialized = false;
        if (!initialized && m_fPlugInitialize) {
            m_fPlugInitialize();
        }
        return "RBN Helper";
    };

    void DrawFrontEndPage(void) {
        if (m_fDrawFrontEndPage) {
            return m_fDrawFrontEndPage();
        }
    };

    void DrawResultsUI(void) {
        if (m_fDrawResultUI) {
            return m_fDrawResultUI();
        }
    };

    void HandleFrontEndEvents(char txtKeyboard, bool bUp, bool bDown, bool bLeft, bool bRight, bool bSelect) {
        if (m_fHandleFrondendEvents) {
            return m_fHandleFrondendEvents(txtKeyboard, bUp, bDown, bLeft, bRight, bSelect);
        }
    };

    void TickFrontEndPage(float fTimeDelta) {
        if (m_fTickFrontEndPage) {
            return m_fTickFrontEndPage(fTimeDelta);
        }
    };

    void StageStarted(int iMap, const char* ptxtPlayerName, bool bWasFalseStart) {
        if (m_fStageStarted) {
            std::string playerName(ptxtPlayerName);
            return m_fStageStarted(iMap, playerName, bWasFalseStart);
        }
    };

    void HandleResults(float fCheckPoint1, float fCheckPoint2, float fFinishTime, const char* ptxtPlayerName) {
        if (m_fHandleResults) {
            std::string playerName(ptxtPlayerName);
            return m_fHandleResults(fCheckPoint1, fCheckPoint2, fFinishTime, playerName);
        }
    };

    void CheckPoint(float fCheckPointTime, int iCheckPointID, const char* ptxtPlayerName) {
        if (m_fCheckPoint) {
            std::string playerName(ptxtPlayerName);
            return m_fCheckPoint(fCheckPointTime, iCheckPointID, playerName);
        }
    };

public:
    PlugInitialize m_fPlugInitialize;
    PlugDrawFrontEndPage m_fDrawFrontEndPage;
    PlugDrawResultUI m_fDrawResultUI;
    PlugHandleFrontEndEvents m_fHandleFrondendEvents;
    PlugTickFrontEndPage m_fTickFrontEndPage;
    PlugStageStarted m_fStageStarted;
    PlugHandleResults m_fHandleResults;
    PlugCheckPoint m_fCheckPoint;

private:
    IRBRGame* m_pGame;
};

static PluginRust *g_pRBRPlugin = nullptr;

#ifdef _WIN32
#define EXPORT_API __declspec(dllexport)
#else
#define EXPORT_API
#endif

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

#ifdef __cplusplus
}
#endif