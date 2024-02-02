#ifndef _IRUST_H_
#define _IRUST_H_

#include <windows.h>
#include <string>
#include "IPlugin.h"
#include "IRBRGame.h"
#include "HookRBR.h"

extern "C" {
    typedef void (*PlugInitialize)(void);
    typedef void (*PlugDrawFrontEndPage)(void);
    typedef void (*PlugDrawResultUI)(void);
    typedef void (*PlugHandleFrontEndEvents)(char txtKeyboard, bool bUp, bool bDown, bool bLeft, bool bRight, bool bSelect);
    typedef void (*PlugTickFrontEndPage)(float fTimeDelta);
    typedef void (*PlugStageStarted)(int iMap, std::string ptxtPlayerName, bool bWasFalseStart);
    typedef void (*PlugHandleResults)(float fCheckPoint1, float fCheckPoint2, float fFinishTime, std::string ptxtPlayerName);
    typedef void (*PlugCheckPoint)(float fCheckPointTime, int iCheckPointID, std::string ptxtPlayerName);
    typedef void (*PlugOnGameModeChanged)(void);
    typedef void (*PlugOnFrame)(void);
    typedef void (*PlugOnBeginScene)(void);
    typedef void (*PlugOnEndScene)(void);
    typedef void (*PlugOnStartRace)(void);
}

class PluginRust : public IPlugin {
public:
    PluginRust(IRBRGame* pGame)
        : m_fPlugInitialize(NULL), m_fDrawFrontEndPage(NULL), m_fDrawResultUI(NULL)
        , m_fHandleFrondendEvents(NULL), m_fTickFrontEndPage(NULL), m_fStageStarted(NULL)
        , m_fHandleResults(NULL), m_fCheckPoint(NULL)
        , m_fOnGameModeChanged(NULL), m_fOnBeginScene(NULL), m_fOnEndScene(NULL)
        , m_fOnStartRace(NULL), m_fOnFrame(NULL)
        , m_pGame(pGame) {};
    virtual ~PluginRust() {};

public:
    const char* GetName(void) {
        static bool initialized = false;
        if (!initialized && m_fPlugInitialize) {
            m_fPlugInitialize();
            if (m_fOnGameModeChanged) HookOnGameModeChanged();
            if (m_fOnFrame) HookOnFrame();
            if (m_fOnBeginScene) HookOnBeginScene();
            if (m_fOnEndScene) HookOnEndScene();
            if (m_fOnStartRace) HookOnStartRace();
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

    void OnGameModeChanged(void) {
        if (m_fOnGameModeChanged) {
            m_fOnGameModeChanged();
        }
    }

    void OnBeginScene(void) {
        if (m_fOnBeginScene) {
            m_fOnBeginScene();
        }
    }

    void OnEndScene(void) {
        if (m_fOnEndScene) {
            m_fOnEndScene();
        }
    }

    void OnStartRace(void) {
        if (m_fOnStartRace) {
            m_fOnStartRace();
        }
    }

    void OnFrame(void) {
        if (m_fOnFrame) {
            m_fOnFrame();
        }
    }

public:
    PlugInitialize m_fPlugInitialize;
    PlugDrawFrontEndPage m_fDrawFrontEndPage;
    PlugDrawResultUI m_fDrawResultUI;
    PlugHandleFrontEndEvents m_fHandleFrondendEvents;
    PlugTickFrontEndPage m_fTickFrontEndPage;
    PlugStageStarted m_fStageStarted;
    PlugHandleResults m_fHandleResults;
    PlugCheckPoint m_fCheckPoint;

    PlugOnGameModeChanged m_fOnGameModeChanged;
    PlugOnFrame m_fOnFrame;
    PlugOnBeginScene m_fOnBeginScene;
    PlugOnEndScene m_fOnEndScene;
    PlugOnStartRace m_fOnStartRace;

private:
    IRBRGame* m_pGame;
};

#endif