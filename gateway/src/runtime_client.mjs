// Explicit standalone Holochain client used by the field gateway and test harness.
// Ports are supplied at runtime; none are embedded in source.

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

export class RuntimeClient {
  constructor({
    adminPort,
    appPort,
    appId = "prometheus",
    role = "hearth",
    zome = "zome_coordinator",
    origin = "http://localhost",
  } = {}) {
    if (!Number(adminPort)) throw new Error("runtime: adminPort is required");
    this.adminPort = Number(adminPort);
    this.appPort = Number(appPort || 0);
    this.appId = appId;
    this.role = role;
    this.zome = zome;
    this.origin = origin;
    this.admin = null;
    this.app = null;
    this.cellId = null;
  }

  async connect() {
    if (this.app && this.cellId) return;
    const { AdminWebsocket, AppWebsocket } = await import("@holochain/client");
    const opts = { origin: this.origin };
    this.admin = await AdminWebsocket.connect({
      url: new URL(`ws://127.0.0.1:${this.adminPort}`),
      wsClientOptions: opts,
    });
    if (!this.appPort) {
      this.appPort = (
        await this.admin.attachAppInterface({ allowed_origins: this.origin })
      ).port;
    }
    const issued = await this.admin.issueAppAuthenticationToken({
      installed_app_id: this.appId,
    });
    this.app = await AppWebsocket.connect({
      url: new URL(`ws://127.0.0.1:${this.appPort}`),
      token: issued.token,
      wsClientOptions: opts,
    });
    const info = await this.app.appInfo();
    const cells = info?.cell_info?.[this.role];
    if (!Array.isArray(cells) || cells.length === 0) {
      throw new Error(`runtime: role "${this.role}" not found in "${this.appId}"`);
    }
    const selected = cells.find((cell) => cell.type === "provisioned") || cells[0];
    this.cellId = (selected.value || selected).cell_id;
    if (!this.cellId) throw new Error(`runtime: no cell_id for role "${this.role}"`);
    let granted = false;
    for (let attempt = 1; attempt <= 8 && !granted; attempt++) {
      try {
        await this.admin.authorizeSigningCredentials(this.cellId);
        granted = true;
      } catch (error) {
        if (attempt === 8) throw error;
        await sleep(2000);
      }
    }
  }

  async call(fnName, payload) {
    try {
      await this.connect();
      return await this.app.callZome({
        cell_id: this.cellId,
        zome_name: this.zome,
        fn_name: fnName,
        payload,
      });
    } catch (error) {
      await this.close();
      throw error;
    }
  }

  async close() {
    if (this.app) {
      try { await this.app.client.close(); } catch {}
    }
    if (this.admin) {
      try { await this.admin.client.close(); } catch {}
    }
    this.app = null;
    this.admin = null;
    this.cellId = null;
  }
}
