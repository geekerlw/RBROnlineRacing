#include <windows.h>
#include <assert.h>
#include "IRust.h"
#include "HookRBR.h"

extern PluginRust *g_pRBRPlugin;
static DWORD g_dwRBRBaseAddress = 0;

#define C_RBR_ADDR_TO_POINTER(absoluteAddr) (g_dwRBRBaseAddress + ((DWORD) absoluteAddr - 0x400000) )

#define C_PROCESS_READ_WRITE_QUERY (PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION | PROCESS_QUERY_INFORMATION)

union BYTEBUFFER_FLOAT {
#pragma pack(push,1)
	float fValue;
	BYTE byteBuffer[sizeof(float)];
	DWORD dwordBuffer;
#pragma pack(pop)
};

union BYTEBUFFER_DWORD {
#pragma pack(push,1)
	DWORD dwValue;
	BYTE byteBuffer[sizeof(DWORD)];
#pragma pack(pop)
};

union BYTEBUFFER_INT32 {
#pragma pack(push,1)
	__int32 iValue;
	BYTE byteBuffer[sizeof(__int32)];
#pragma pack(pop)
};

union BYTEBUFFER_PTR {
#pragma pack(push,1)
	LPVOID ptrValue;
	BYTE byteBuffer[sizeof(LPVOID)];
#pragma pack(pop)
};

//----------------------------------------------------------------------------------------------------------------------------
// Helper functions to modify RBR memory locations on the fly
//

// Get a base address of a specified module (if it is already loaded)
LPVOID GetModuleBaseAddr(const char* szModuleName)
{
	HMODULE hModule = nullptr;
	if (::GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, szModuleName, &hModule))
		return (LPVOID)hModule;
	else
		return nullptr;
}

LPVOID GetModuleOffsetAddr(const char* szModuleName, DWORD offset)
{
	LPVOID pBaseAddr = ::GetModuleBaseAddr(szModuleName);
	if (pBaseAddr != nullptr)
		pBaseAddr = (LPVOID)(((DWORD)pBaseAddr) + offset);
	return pBaseAddr;
}

// Convert hex char to int value
int char2int(const char input)
{
	if (input >= '0' && input <= '9') return input - '0';
	if (input >= 'A' && input <= 'F') return input - 'A' + 10;
	if (input >= 'a' && input <= 'f') return input - 'a' + 10;

	// Should never come here
	assert(1 == 0);
	return 0;
}

// Convert string with HEX number values as byteBuffer (the string must have even number of chars and valid hex chars)
void hexString2byteArray(LPCSTR sHexText, BYTE* byteBuffer)
{
	while (*sHexText && sHexText[1])
	{
		*(byteBuffer++) = char2int(*sHexText) * 0x10 + char2int(sHexText[1]);
		sHexText += 2;
	}
}

// Write buffer to specified memory location
BOOL WriteOpCodeBuffer(const LPVOID writeAddr, const BYTE* buffer, const int iBufLen)
{
	HANDLE hProcess;
	DWORD  dwOldProtectValue;
	BOOL bResult;

	if (writeAddr == nullptr)
		return FALSE;

	hProcess = OpenProcess(C_PROCESS_READ_WRITE_QUERY, FALSE, GetCurrentProcessId());
	bResult = VirtualProtectEx(hProcess, writeAddr, iBufLen, PAGE_READWRITE, &dwOldProtectValue);
	if (bResult) bResult = WriteProcessMemory(hProcess, writeAddr, buffer, iBufLen, 0);
	CloseHandle(hProcess);

	return bResult;

}

// Read the value of specified memory location and return in buffer
BOOL ReadOpCodeBuffer(const LPVOID readAddr, BYTE* buffer, const int iBufLen)
{
	HANDLE hProcess;
	DWORD  dwOldProtectValue;
	BOOL bResult;

	hProcess = OpenProcess(C_PROCESS_READ_WRITE_QUERY, FALSE, GetCurrentProcessId());
	bResult = VirtualProtectEx(hProcess, readAddr, iBufLen, PAGE_READWRITE, &dwOldProtectValue);
	if (bResult) bResult = ReadProcessMemory(hProcess, readAddr, buffer, iBufLen, 0);
	CloseHandle(hProcess);

	return bResult;

}

BOOL WriteOpCodeHexString(const LPVOID writeAddr, LPCSTR sHexText)
{
	BYTE byteBuffer[64];
	int iLen = strlen(sHexText) / 2;

	assert(iLen <= 64);
	hexString2byteArray(sHexText, byteBuffer);

	return WriteOpCodeBuffer(writeAddr, byteBuffer, iLen);
}

BOOL WriteOpCodeInt32(const LPVOID writeAddr, const __int32 iValue)
{
	BYTEBUFFER_INT32 dataUnionInt32{};
	dataUnionInt32.iValue = iValue;

	return WriteOpCodeBuffer(writeAddr, dataUnionInt32.byteBuffer, sizeof(__int32));
}

BOOL ReadOpCodeInt32(const LPVOID readAddr, __int32* iValue)
{
	BYTEBUFFER_INT32 dataUnionInt32{};
	if (ReadOpCodeBuffer(readAddr, dataUnionInt32.byteBuffer, sizeof(__int32)))
	{
		*iValue = dataUnionInt32.iValue;
		return TRUE;
	}
	else return FALSE;
}

BOOL WriteOpCodeFloat(const LPVOID writeAddr, const float fValue)
{
	BYTEBUFFER_FLOAT dataUnionFloat{};
	dataUnionFloat.fValue = fValue;

	return WriteOpCodeBuffer(writeAddr, dataUnionFloat.byteBuffer, sizeof(float));
}

BOOL ReadOpCodeFloat(const LPVOID readAddr, float* fValue)
{
	BYTEBUFFER_FLOAT dataUnionFloat{};
	if (ReadOpCodeBuffer(readAddr, dataUnionFloat.byteBuffer, sizeof(float)))
	{
		*fValue = dataUnionFloat.fValue;
		return TRUE;
	}
	else return FALSE;
}

BOOL WriteOpCodePtr(const LPVOID writeAddr, const LPVOID ptrValue)
{
	BYTEBUFFER_PTR dataUnionPtr{};
	dataUnionPtr.ptrValue = ptrValue;

	return WriteOpCodeBuffer(writeAddr, dataUnionPtr.byteBuffer, sizeof(LPVOID));
}

BOOL ReadOpCodePtr(const LPVOID readAddr, LPVOID* ptrValue)
{
	BYTEBUFFER_PTR dataUnionPtr{};
	if (ReadOpCodeBuffer(readAddr, dataUnionPtr.byteBuffer, sizeof(LPVOID)))
	{
		*ptrValue = dataUnionPtr.ptrValue;
		return TRUE;
	}
	else return FALSE;
}

BOOL WriteOpCodeByte(const LPVOID writeAddr, const BYTE byteValue)
{
	return WriteOpCodeBuffer(writeAddr, &byteValue, sizeof(BYTE));
}

BOOL ReadOpCodeByte(const LPVOID readAddr, BYTE byteValue)
{
	return ReadOpCodeBuffer(readAddr, &byteValue, sizeof(BYTE));
}

BOOL WriteOpCodeNearCallCmd(const LPVOID writeAddr, const LPVOID callTargetAddr)
{
	// TODO: Rel16 vs Rel32 logic based on the call distance?
	BYTE buffer[5]{};
	DWORD callOffset = ((DWORD)callTargetAddr) - (((DWORD)writeAddr) + 5);
	buffer[0] = 0xE8;
	buffer[1] = (BYTE)((callOffset & 0x000000FF));
	buffer[2] = (BYTE)((callOffset & 0x0000FF00) >> 8);
	buffer[3] = (BYTE)((callOffset & 0x00FF0000) >> 16);
	buffer[4] = (BYTE)((callOffset & 0xFF000000) >> 24);
	return WriteOpCodeBuffer(writeAddr, buffer, 5);
}

BOOL WriteOpCodeLongCallCmd(const LPVOID writeAddr, const LPVOID jmpTargetAddr, BOOL addTrailingNOP)
{
	BYTE buffer[6]{};
	DWORD callOffset = ((DWORD)jmpTargetAddr) - (((DWORD)writeAddr) + 5);
	buffer[0] = 0xE8;
	buffer[1] = (BYTE)((callOffset & 0x000000FF));
	buffer[2] = (BYTE)((callOffset & 0x0000FF00) >> 8);
	buffer[3] = (BYTE)((callOffset & 0x00FF0000) >> 16);
	buffer[4] = (BYTE)((callOffset & 0xFF000000) >> 24);
	buffer[5] = 0x90;
	return WriteOpCodeBuffer(writeAddr, buffer, (addTrailingNOP ? 6 : 5));
}

BOOL WriteOpCodeNearJmpCmd(const LPVOID writeAddr, const LPVOID jmpTargetAddr)
{
	// TODO: Rel8 vs Rel16 vs Rel32 logic based on the call distance?
	BYTE buffer[2]{};
	DWORD callOffset = ((DWORD)jmpTargetAddr) - (((DWORD)writeAddr) + 2);
	buffer[0] = 0xEB;
	buffer[1] = (BYTE)((callOffset & 0x000000FF));
	return WriteOpCodeBuffer(writeAddr, buffer, 2);
}

BOOL WriteOpCodeLongJmpCmd(const LPVOID writeAddr, const LPVOID jmpTargetAddr, BOOL addTrailingNOP)
{
	BYTE buffer[6]{};
	DWORD callOffset = ((DWORD)jmpTargetAddr) - (((DWORD)writeAddr) + 5);
	buffer[0] = 0xE9;
	buffer[1] = (BYTE)((callOffset & 0x000000FF));
	buffer[2] = (BYTE)((callOffset & 0x0000FF00) >> 8);
	buffer[3] = (BYTE)((callOffset & 0x00FF0000) >> 16);
	buffer[4] = (BYTE)((callOffset & 0xFF000000) >> 24);
	buffer[5] = 0x90;
	return WriteOpCodeBuffer(writeAddr, buffer, (addTrailingNOP ? 6 : 5));
}

//------------------------------------------------------------------------------------------

/// <summary>
/// RBR On-Frame handler in racing and replays (this is called when a race countdown starts running)
/// </summary>
static void OnFrame()
{
	g_pRBRPlugin->OnFrame();
}

static LPVOID g_OnFrameEAXPtr = nullptr;
static LPVOID g_OnFramePrevHandler = nullptr;

static void __declspec(naked) OnFrameProxy()
{
	__asm
	{
		push ebx
		push esi
		push edi

		call OnFrame

		pop  edi
		pop  esi
		pop  ebx

		mov eax, dword ptr ds : [g_OnFrameEAXPtr]
		jmp dword ptr ds : [g_OnFramePrevHandler]
	}
}

void HookOnFrame()
{
	static bool hookAlreadyInstalled = false;
	if (hookAlreadyInstalled)
		return;

	// Initialize OnFrame hook only once. If there is already a custom hook then call it as the previous handler (other plugins may hook OnFrame also)
	hookAlreadyInstalled = true;

	if (*(BYTE*)C_RBR_ADDR_TO_POINTER(0x578CDF) == 0xA1)
	{
		// No other custom OnFrame hooks. Use the default RBR handler in our own OnFrame proxy handler as the previous handler
		g_OnFramePrevHandler = (LPVOID)C_RBR_ADDR_TO_POINTER(0x578CE4);
	}
	else
	{
		// Custom OnFrame already set. OnFrame proxy handler uses it as the previous handler
		g_OnFramePrevHandler = (LPVOID)(((DWORD)C_RBR_ADDR_TO_POINTER(0x578CE4)) + *((DWORD*)C_RBR_ADDR_TO_POINTER(0x578CE0)));
	}

	g_OnFrameEAXPtr = (LPVOID)C_RBR_ADDR_TO_POINTER(0x01660CE8);
	WriteOpCodeLongJmpCmd((LPVOID)C_RBR_ADDR_TO_POINTER(0x578CDF), &OnFrameProxy, false);
}

//------------------------------------------------------------------------------------------

/// <summary>
/// Called when the DX9 frame is about to be drawn
/// </summary>
static void OnBeginScene()
{
	g_pRBRPlugin->OnBeginScene();
}

static LPVOID g_OnBeginScenePrevHandler = nullptr;

static void __declspec(naked) OnBeginSceneProxy()
{
	__asm
	{
		push ecx

		mov eax, dword ptr ds : [ecx + 0xF4]
		call dword ptr ds : [g_OnBeginScenePrevHandler]

		call OnBeginScene

		pop ecx

		ret
	}

	/*
	__asm
	{
		push ecx

		call OnBeginScene

		pop ecx

		mov eax, dword ptr ds : [ecx + 0xF4]
		jmp dword ptr ds : [g_OnBeginScenePrevHandler]
	}
	*/
}

void HookOnBeginScene()
{
	static bool hookAlreadyInstalled = false;
	if (hookAlreadyInstalled)
		return;

	hookAlreadyInstalled = true;

	if (*((BYTE*)C_RBR_ADDR_TO_POINTER(0x40E880)) == 0x8B)
	{
		// No other custom hooks. Use the default RBR handler as the previous handler
		g_OnBeginScenePrevHandler = (LPVOID)C_RBR_ADDR_TO_POINTER(0x40E886);
	}
	else
	{
		// Custom handler already set. Link this and the previous custom handler
		g_OnBeginScenePrevHandler = (LPVOID)(((DWORD)C_RBR_ADDR_TO_POINTER(0x40E885)) + *((DWORD*)C_RBR_ADDR_TO_POINTER(0x40E881)));
	}

	WriteOpCodeLongJmpCmd((LPVOID)C_RBR_ADDR_TO_POINTER(0x40E880), &OnBeginSceneProxy, true);
}

//------------------------------------------------------------------------------------------

/// <summary>
/// Called when the DX9 frame is completed and posted to GPU
/// </summary>
static void OnEndScene()
{
	g_pRBRPlugin->OnEndScene();
}

static LPVOID g_OnEndScenePrevHandler = nullptr;

static void __declspec(naked) OnEndSceneProxy()
{
	__asm
	{
		push ecx
		push edx

		call OnEndScene

		pop edx
		pop ecx

		xor eax, eax
		jmp dword ptr ds : [g_OnEndScenePrevHandler]
	}
}

void HookOnEndScene()
{
	static bool hookAlreadyInstalled = false;
	if (hookAlreadyInstalled)
		return;

	hookAlreadyInstalled = true;

	if (*((BYTE*)C_RBR_ADDR_TO_POINTER(0x40E896)) == 0x33)
	{
		// No other custom hooks. Use the default RBR handler as the previous handler
		g_OnEndScenePrevHandler = (LPVOID)C_RBR_ADDR_TO_POINTER(0x40E89B);
	}
	else
	{
		// Custom handler already set. Link this and the previous custom handler
		g_OnEndScenePrevHandler = (LPVOID)(((DWORD)C_RBR_ADDR_TO_POINTER(0x40E89B)) + *((DWORD*)C_RBR_ADDR_TO_POINTER(0x40E897)));
	}

	WriteOpCodeLongJmpCmd((LPVOID)C_RBR_ADDR_TO_POINTER(0x40E896), &OnEndSceneProxy, false);
}

//------------------------------------------------------------------------------------------

/// <summary>
/// Called when the game mode changed
/// </summary>
static void OnGameModeChanged()
{
	g_pRBRPlugin->OnGameModeChanged();
}

//
// OnGameModeChanged = "Game mode changed" handler when RBR changes a game mode (menu, map loaded, stage preparing (cam spinning), racing or replaying, pause menu, back to menu etc)
// 0x01 = Racing
// 0x02 = Race time pause menu
// 0x03 = RBR menu or custom plugin menu
// 0x08 = Replaying
// 0x0A = Race/Replay ready to start (cam spinning around the car)
// 0x0D = Map loaded
//
static LPVOID g_OnGameModeChangedPrevHandler = nullptr;

static void __declspec(naked) OnGameModeChangedProxy()
{
	__asm
	{
		mov dword ptr ds : [esi + 0x728] , edi

		push esi
		push edi
		
		call OnGameModeChanged

		pop edi
		pop esi

		jmp dword ptr ds : [g_OnGameModeChangedPrevHandler]
	}
}

void HookOnGameModeChanged()
{
	static bool hookAlreadyInstalled = false;
	if (hookAlreadyInstalled)
		return;

	hookAlreadyInstalled = true;

	if (*((BYTE*)C_RBR_ADDR_TO_POINTER(0x47F392)) == 0x89)
	{
		// No other custom hooks. Use the default RBR handler as the previous handler
		g_OnGameModeChangedPrevHandler = (LPVOID)C_RBR_ADDR_TO_POINTER(0x47F398);
	}
	else
	{
		// Custom handler already set. Link this and the previous custom handler
		g_OnGameModeChangedPrevHandler = (LPVOID)(((DWORD)C_RBR_ADDR_TO_POINTER(0x47F397)) + *((DWORD*)C_RBR_ADDR_TO_POINTER(0x47F393)));
	}

	WriteOpCodeLongJmpCmd((LPVOID)C_RBR_ADDR_TO_POINTER(0x47F392), &OnGameModeChangedProxy, true);
}

//----------------------------------------------------------------------------------------

void OnStartRace()
{
	g_pRBRPlugin->OnStartRace();
}

void HookOnStartRace()
{
	// TODO: This hook works only RSF installation. Fix this to support RBRTM also, but until them this hook doesn't do anything in RBRTM.
	// Anyway, this plugin does work in RBRTM also, but user just have to set gearbox to manual mode and disable all clutchHelp options in RBR settings.
	static bool hookAlreadyInstalled = false;
	if (hookAlreadyInstalled || GetModuleBaseAddr("Rallysimfans.hu.dll") == nullptr)
		return;

	WriteOpCodeLongCallCmd((LPVOID)C_RBR_ADDR_TO_POINTER(0x626AAC), &OnStartRace, true);
}
