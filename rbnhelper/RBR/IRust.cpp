#include "IPlugin.h"
#include "IRBRGame.h"

class PluginRust : public IPlugin {
public:
    PluginRust() {};
    virtual ~PluginRust() {};

public:
    const char* GetName(void) {return "RBN Helper";};
    void DrawFrontEndPage(void) {};
    void DrawResultsUI(void) {};
    void HandleFrontEndEvents(char txtKeyboard, bool bUp, bool bDown, bool bLeft, bool bRight, bool bSelect) {};
    void TickFrontEndPage(float fTimeDelta) {};
    void StageStarted(int iMap, const char* ptxtPlayerName, bool bWasFalseStart) {};
    void HandleResults(float fCheckPoint1, float fCheckPoint2, float fFinishTime, const char* ptxtPlayerName) {};
    void CheckPoint(float fCheckPointTime, int iCheckPointID, const char* ptxtPlayerName) {};
};

#ifdef _WIN32
#define EXPORT_API __declspec(dllexport)
#else
#define EXPORT_API
#endif

#ifdef __cplusplus
extern "C" {
#endif

EXPORT_API int test_function() {
    return 3;
}

#ifdef __cplusplus
}
#endif