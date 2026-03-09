import { afterEach, describe, expect, it, vi } from "vitest";

const { invokeMock } = vi.hoisted(() => ({
  invokeMock: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: invokeMock,
}));

import { failoverApi } from "@/lib/api/failover";
import { omoApi, omoSlimApi } from "@/lib/api/omo";
import { openclawApi } from "@/lib/api/openclaw";
import { proxyApi } from "@/lib/api/proxy";
import { getGlobalProxyUrl, scanLocalProxies, setGlobalProxyUrl } from "@/lib/api/globalProxy";
import { providersApi, universalProvidersApi } from "@/lib/api/providers";
import { sessionsApi } from "@/lib/api/sessions";
import { settingsApi } from "@/lib/api/settings";
import { skillsApi } from "@/lib/api/skills";
import { usageApi } from "@/lib/api/usage";
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

  it("maps failover queue and auto-failover helpers to current commands", async () => {
    invokeMock.mockResolvedValue([]);
    await failoverApi.getFailoverQueue("claude");
    expect(invokeMock).toHaveBeenNthCalledWith(1, "get_failover_queue", {
      appType: "claude",
    });

    invokeMock.mockResolvedValue([]);
    await failoverApi.getAvailableProvidersForFailover("claude");
    expect(invokeMock).toHaveBeenNthCalledWith(2, "get_available_providers_for_failover", {
      appType: "claude",
    });

    invokeMock.mockResolvedValue(undefined);
    await failoverApi.addToFailoverQueue("claude", "provider-1");
    expect(invokeMock).toHaveBeenNthCalledWith(3, "add_to_failover_queue", {
      appType: "claude",
      providerId: "provider-1",
    });

    invokeMock.mockResolvedValue(undefined);
    await failoverApi.removeFromFailoverQueue("claude", "provider-1");
    expect(invokeMock).toHaveBeenNthCalledWith(4, "remove_from_failover_queue", {
      appType: "claude",
      providerId: "provider-1",
    });

    invokeMock.mockResolvedValue(false);
    await failoverApi.getAutoFailoverEnabled("claude");
    expect(invokeMock).toHaveBeenNthCalledWith(5, "get_auto_failover_enabled", {
      appType: "claude",
    });

    invokeMock.mockResolvedValue(true);
    await failoverApi.setAutoFailoverEnabled("claude", true);
    expect(invokeMock).toHaveBeenNthCalledWith(6, "set_auto_failover_enabled", {
      appType: "claude",
      enabled: true,
    });
  });

  it("maps proxy runtime commands to current Tauri names", async () => {
    invokeMock.mockResolvedValue(undefined);

    await proxyApi.startProxyServer();
    await proxyApi.getProxyStatus();
    await proxyApi.isProxyRunning();
    await proxyApi.isLiveTakeoverActive();
    await proxyApi.setProxyTakeoverForApp("claude", true);
    await proxyApi.switchProxyProvider("claude", "provider-2");
    await proxyApi.stopProxyWithRestore();

    expect(invokeMock).toHaveBeenNthCalledWith(1, "start_proxy_server");
    expect(invokeMock).toHaveBeenNthCalledWith(2, "get_proxy_status");
    expect(invokeMock).toHaveBeenNthCalledWith(3, "is_proxy_running");
    expect(invokeMock).toHaveBeenNthCalledWith(4, "is_live_takeover_active");
    expect(invokeMock).toHaveBeenNthCalledWith(5, "set_proxy_takeover_for_app", {
      appType: "claude",
      enabled: true,
    });
    expect(invokeMock).toHaveBeenNthCalledWith(6, "switch_proxy_provider", {
      appType: "claude",
      providerId: "provider-2",
    });
    expect(invokeMock).toHaveBeenNthCalledWith(7, "stop_proxy_with_restore");
  });

  it("maps usage detail and pricing helpers to current commands", async () => {
    invokeMock.mockResolvedValue(null);

    await usageApi.getRequestDetail("req-1");
    expect(invokeMock).toHaveBeenNthCalledWith(1, "get_request_detail", {
      requestId: "req-1",
    });

    invokeMock.mockResolvedValue(undefined);
    await usageApi.updateModelPricing(
      "model-1",
      "Model One",
      "1.0",
      "2.0",
      "0.1",
      "0.2",
    );
    expect(invokeMock).toHaveBeenNthCalledWith(2, "update_model_pricing", {
      modelId: "model-1",
      displayName: "Model One",
      inputCost: "1.0",
      outputCost: "2.0",
      cacheReadCost: "0.1",
      cacheCreationCost: "0.2",
    });

    invokeMock.mockResolvedValue([]);
    await usageApi.getModelPricing();
    expect(invokeMock).toHaveBeenNthCalledWith(3, "get_model_pricing");

    invokeMock.mockResolvedValue(undefined);
    await usageApi.deleteModelPricing("model-1");
    expect(invokeMock).toHaveBeenNthCalledWith(4, "delete_model_pricing", {
      modelId: "model-1",
    });
  });

  it("maps universal provider commands to current Tauri names", async () => {
    invokeMock.mockResolvedValue({});
    await universalProvidersApi.getAll();
    await universalProvidersApi.get("uni-1");
    await universalProvidersApi.upsert({
      id: "uni-1",
      name: "Universal One",
      providerType: "newapi",
      apps: {
        claude: true,
        codex: false,
        gemini: false,
      },
      baseUrl: "https://api.example.com",
      apiKey: "api-key-1",
      models: {},
    });
    await universalProvidersApi.sync("uni-1");
    await universalProvidersApi.delete("uni-1");

    expect(invokeMock).toHaveBeenNthCalledWith(1, "get_universal_providers");
    expect(invokeMock).toHaveBeenNthCalledWith(2, "get_universal_provider", {
      id: "uni-1",
    });
    expect(invokeMock).toHaveBeenNthCalledWith(3, "upsert_universal_provider", {
      provider: {
        id: "uni-1",
        name: "Universal One",
        providerType: "newapi",
        apps: {
          claude: true,
          codex: false,
          gemini: false,
        },
        baseUrl: "https://api.example.com",
        apiKey: "api-key-1",
        models: {},
      },
    });
    expect(invokeMock).toHaveBeenNthCalledWith(4, "sync_universal_provider", {
      id: "uni-1",
    });
    expect(invokeMock).toHaveBeenNthCalledWith(5, "delete_universal_provider", {
      id: "uni-1",
    });
  });

  it("maps openclaw config helpers to current commands", async () => {
    invokeMock.mockResolvedValue(undefined);

    await openclawApi.setDefaultModel({
      primary: "provider-a/model-a",
      fallbacks: ["provider-b/model-b"],
    });
    await openclawApi.setModelCatalog({
      "provider-a/model-a": { alias: "primary" },
    });
    await openclawApi.setAgentsDefaults({
      model: {
        primary: "provider-a/model-a",
        fallbacks: ["provider-b/model-b"],
      },
      models: {
        "provider-a/model-a": { alias: "primary" },
      },
    });
    await openclawApi.setEnv({
      ANTHROPIC_API_KEY: "test-key",
    });
    await openclawApi.setTools({
      profile: "strict",
      allow: ["Read(*)"],
      deny: ["Shell(*)"],
    });

    expect(invokeMock).toHaveBeenNthCalledWith(1, "set_openclaw_default_model", {
      model: {
        primary: "provider-a/model-a",
        fallbacks: ["provider-b/model-b"],
      },
    });
    expect(invokeMock).toHaveBeenNthCalledWith(2, "set_openclaw_model_catalog", {
      catalog: {
        "provider-a/model-a": { alias: "primary" },
      },
    });
    expect(invokeMock).toHaveBeenNthCalledWith(3, "set_openclaw_agents_defaults", {
      defaults: {
        model: {
          primary: "provider-a/model-a",
          fallbacks: ["provider-b/model-b"],
        },
        models: {
          "provider-a/model-a": { alias: "primary" },
        },
      },
    });
    expect(invokeMock).toHaveBeenNthCalledWith(4, "set_openclaw_env", {
      env: {
        ANTHROPIC_API_KEY: "test-key",
      },
    });
    expect(invokeMock).toHaveBeenNthCalledWith(5, "set_openclaw_tools", {
      tools: {
        profile: "strict",
        allow: ["Read(*)"],
        deny: ["Shell(*)"],
      },
    });
  });

  it("maps omo commands to current Tauri names", async () => {
    invokeMock.mockResolvedValue(undefined);

    await omoApi.getCurrentOmoProviderId();
    await omoApi.disableCurrentOmo();
    await omoSlimApi.getCurrentProviderId();
    await omoSlimApi.disableCurrent();

    expect(invokeMock).toHaveBeenNthCalledWith(1, "get_current_omo_provider_id");
    expect(invokeMock).toHaveBeenNthCalledWith(2, "disable_current_omo");
    expect(invokeMock).toHaveBeenNthCalledWith(3, "get_current_omo_slim_provider_id");
    expect(invokeMock).toHaveBeenNthCalledWith(4, "disable_current_omo_slim");
  });
});
