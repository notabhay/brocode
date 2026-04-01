import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, beforeEach } from "@jest/globals";

const originalBrocodeHome = process.env.BROCODE_HOME;
let currentBrocodeHome: string | undefined;

beforeEach(async () => {
  currentBrocodeHome = await fs.mkdtemp(path.join(os.tmpdir(), "brocode-sdk-test-"));
  process.env.BROCODE_HOME = currentBrocodeHome;
});

afterEach(async () => {
  const brocodeHomeToDelete = currentBrocodeHome;
  currentBrocodeHome = undefined;

  if (originalBrocodeHome === undefined) {
    delete process.env.BROCODE_HOME;
  } else {
    process.env.BROCODE_HOME = originalBrocodeHome;
  }

  if (brocodeHomeToDelete) {
    await fs.rm(brocodeHomeToDelete, { recursive: true, force: true });
  }
});
