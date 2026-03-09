import { afterEach, describe, expect, it, vi } from "vitest";

const { invokeMock } = vi.hoisted(() => ({
  invokeMock: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: invokeMock,
}));

import { getGlobalProxyUrl, scanLocalProxies, setGlobalProxyUrl } from "@/lib/api/globalProxy";
import { providersApi } from "@/lib/api/providers";
import { sessionsApi } from "@/lib/api/sessions";
import { settingsApi } from "@/lib/api/settings";
import { skillsApi } from "@/lib/api/skills";
import { workspaceApi } from "@/lib/api/workspace";

afterEach(() => {
  invokeMock.mockReset();
});

describe("Tauri API contracts", () => {
  it("maps provider switch to the current Tauri command", async () => {
    invokeMock.mockResolvedValue({ warnings: [] });

    await providersApi.switch("provider-1", "claude");

    expect(invokeMock).toHaveBeenCalledWith("switch_provider", {
      id: "provider-1",
      app: "claude",
    });
  });

  it("maps settings save to save_settings", async () => {
    invokeMock.mockResolvedValue(true);

    await settingsApi.save({ language: "zh", showInTray: false } as never);

    expect(invokeMock).toHaveBeenCalledWith("save_settings", {
      settings: { language: "zh", showInTray: false },
    });
  });

  it("maps webdav save to webdav_sync_save_settings", async () => {
    invokeMock.mockResolvedValue({ success: true });

    await settingsApi.webdavSyncSaveSettings(
      {
        enabled: true,
        baseUrl: "https://dav.example.com",
        username: "alice",
        password: "secret",
        remoteRoot: "cc-switch/device",
        autoSync: false,
        profile: "default",
        status: {},
      },
      true,
    );

    expect(invokeMock).toHaveBeenCalledWith("webdav_sync_save_settings", {
      settings: {
        enabled: true,
        baseUrl: "https://dav.example.com",
        username: "alice",
        password: "secret",
        remoteRoot: "cc-switch/device",
        autoSync: false,
        profile: "default",
        status: {},
      },
      passwordTouched: true,
    });
  });

  it("maps workspace daily memory writes to write_daily_memory_file", async () => {
    invokeMock.mockResolvedValue(undefined);

    await workspaceApi.writeDailyMemoryFile("2026-03-09.md", "hello memory");

    expect(invokeMock).toHaveBeenCalledWith("write_daily_memory_file", {
      filename: "2026-03-09.md",
      content: "hello memory",
    });
  });

  it("maps session message loading to get_session_messages", async () => {
    invokeMock.mockResolvedValue([]);

    await sessionsApi.getMessages("codex", "/tmp/demo.jsonl");

    expect(invokeMock).toHaveBeenCalledWith("get_session_messages", {
      providerId: "codex",
      sourcePath: "/tmp/demo.jsonl",
    });
  });

  it("maps skills import to import_skills_from_apps", async () => {
    invokeMock.mockResolvedValue([]);

    await skillsApi.importFromApps(["demo-skill"]);

    expect(invokeMock).toHaveBeenCalledWith("import_skills_from_apps", {
      directories: ["demo-skill"],
    });
  });

  it("maps global proxy helpers to current commands", async () => {
    invokeMock.mockResolvedValueOnce("http://127.0.0.1:7890");
    invokeMock.mockResolvedValueOnce(undefined);
    invokeMock.mockResolvedValueOnce([]);

    await getGlobalProxyUrl();
    await setGlobalProxyUrl("http://127.0.0.1:7890");
    await scanLocalProxies();

    expect(invokeMock).toHaveBeenNthCalledWith(1, "get_global_proxy_url");
    expect(invokeMock).toHaveBeenNthCalledWith(2, "set_global_proxy_url", {
      url: "http://127.0.0.1:7890",
    });
    expect(invokeMock).toHaveBeenNthCalledWith(3, "scan_local_proxies");
  });
});
