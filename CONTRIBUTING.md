CONTRIBUTING.md – Technical Reference for Holochain Developers
A Complete Implementation Guide Referencing the Full h•eart•h Prometheus White Paper
Welcome, fellow builder. This document is the definitive technical guide for contributing to hearth-prometheus-happ. It references every major section, article, and appendix of the white paper, translating them into concrete development practices, code patterns, and validation requirements. Read it before you write your first line of Rust.

Appendices Reference

📋 Table of Contents

Prerequisites & Core Concepts

Repository Structure

Constitutional Coding: The Seven Eternity Clauses

Access Covenant Implementation

Three-Plane Architecture in Code

Operational Plane (Holochain Zomes)

Cognitive Plane (Meta-Prompt Engine & Agents)

Experiential Plane (UI & Mobile)

Value Flows: REA/Valueflows Implementation

Tokenization: TRBK, HOT, and Impact Credits

MRV Protocol: Sensors, Satellites, and Proof of Care

The Immune System: Ravel Software & Regenerative Brake

Human State Interface (HSI) Integration

Governance: Sociocracy, Reputation, and Warrants

Syntropic Sovereignty Header

Testing & Validation

Pull Request Process & Checklist


📜 The Living Constitution: White Paper & Manifesto

h•eart•h Prometheus is not just code. It is a **living system** rooted in a constitutional framework. Before you write a single line of Rust, you must understand the soul of the machine.

Two documents form our foundation. They are not static PDFs, but **cryptographically sealed artifacts**, ensuring that the principles you build with today are the same as those envisioned at the project's genesis.

1. The White Paper (The Architecture of Sovereignty)

This is the complete technical and constitutional blueprint—over 200 pages defining the Eternity Clauses, the Access Covenant, the Three-Plane Architecture, REA flows, and every protocol you will implement.

*   **For Deep Reading:** [**Download the Official White Paper (PDF)**](https://github.com/Uwohali/hearth-prometheus-whitepaper/releases/download/v1.0.2/h•eart•h_Prometheus_White_Paper.pdf)
*   **For Technical Reference (Linking to Sections):** [**View the Markdown Source on GitHub**](https://github.com/Uwohali/hearth-prometheus-whitepaper/blob/v1.0.2/h•eart•h_Prometheus_White_Paper.md)

2. The Manifesto (The Philosophical Heart)

This is the "why." It articulates the vision of Syntropic Sovereignty, the Spiral of Abundance, and our relationship with Nature as a living intelligence. Reading it will attune you to the project's frequency.

*   **For Deep Reading:** [**Download the Official Manifesto (PDF)**](https://github.com/Uwohali/hearth-prometheus-whitepaper/releases/download/v1.0.2/h•eart•h_intelligence_Manifesto.pdf)
*   **For Quick Reference:** [**View the Markdown Source on GitHub**](https://github.com/Uwohali/hearth-prometheus-whitepaper/blob/v1.0.2/h•eart•h_intelligence_Manifesto.md)

---

🔐 The Ritual of Verification: Trust, Don't Trust

In a world of deepfakes and altered information, we establish trust through cryptography. Every official document—both PDF and Markdown—is accompanied by a GPG signature (`.asc` file). You **must** verify them. It's not just a step; it's a declaration of alignment with the source of truth.

Here is the ritual to confirm you are building on an unbroken foundation.

Step 1: Import the Constituent's Key

The documents are signed by Uwohali Tuccio. Import their public key to your keyring.

```bash
curl -O https://github.com/Uwohali/hearth-prometheus-whitepaper/raw/main/Uwohali_Tuccio_PUBLIC_KEY.asc
gpg --import Uwohali_Tuccio_PUBLIC_KEY.asc
```

**Expected Output:**
You should see a message confirming the key was imported. The full fingerprint is:
`D20D FF6C BE33 1B3A 4109 DF84 8C8C B0D4 8C7F 60DA`

Step 2: Verify a Document's Signature

Download a document and its signature file. Then, verify them together.

For example, to verify the White Paper PDF:

```bash
# Download the PDF and its signature
curl -LO https://github.com/Uwohali/hearth-prometheus-whitepaper/releases/download/v1.0.2/h•eart•h_Prometheus_White_Paper.pdf
curl -LO https://github.com/Uwohali/hearth-prometheus-whitepaper/releases/download/v1.0.2/h•eart•h_Prometheus_White_Paper.pdf.asc

# Verify the signature against the PDF
gpg --verify h•eart•h_Prometheus_White_Paper.pdf.asc h•eart•h_Prometheus_White_Paper.pdf
```

**Look for the magic words:** `gpg: Good signature from "Uwohali Tuccio (The Constituent) <uwohali77@gmail.com>"`

⚠️ A Note on Trust: You may see a warning that the key is not certified with a trusted signature. This is normal for a newly imported key. The "Good signature" itself confirms the file hasn't been tampered with since it was signed. The chain of trust begins with your conscious decision to trust this key, whose fingerprint you can verify through other channels.

Step 3: (Optional) Batch Verify All Files with SHA256
For a complete check, you can use the signed checksums file.

bash
# Download the checksums file and its signature
curl -LO https://github.com/Uwohali/hearth-prometheus-whitepaper/raw/v1.0.2/SHA256_SUMS.txt
curl -LO https://github.com/Uwohali/hearth-prometheus-whitepaper/raw/v1.0.2/SHA256_SUMS.txt.asc

# Verify the signature of the checksums file
gpg --verify SHA256_SUMS.txt.asc SHA256_SUMS.txt

# If that's Good, verify the hashes of the files you've downloaded
# On Linux/macOS:
sha256sum -c SHA256_SUMS.txt 2>&1 | grep -E 'OK|FAILED'
# On Windows (PowerShell):
Get-Content SHA256_SUMS.txt | Where-Object { $_ -match '^[A-F0-9]{64}' } | ForEach-Object { $parts = $_ -split '\s+', 2; $expected = $parts[0]; $file = $parts[1]; $computed = (Get-FileHash $file -Algorithm SHA256).Hash.ToUpper(); if ($expected -eq $computed) { Write-Host "✅ $file OK" -ForegroundColor Green } else { Write-Host "❌ $file NOT VALID" -ForegroundColor Red } }
All files should return OK. If any file reports FAILED or NOT VALID, stop immediately and investigate. You may have a corrupted or inauthentic file.

🧩 Technical Extracts: For Coders, By Coders
We know you don't want to scroll through 200 pages to find the definition of a CapitalFlow. For your convenience, key technical sections have been extracted into standalone, linkable Markdown files. These are the same text, just easier to reference while coding.

Appendix I: REA/Valueflows DNA Specification – The core of all economic accounting.

Appendix D: Syntropic Sovereignty Header – The cryptographic stamp for all data.

Appendix G: MRV Protocol – How sensors, satellites, and proof-of-care create trust.

Appendix J: Governance & Sociocracy – Circles, reputation, and warrants.

(More extracts will be added here as the project evolves. Check the /docs folder in the whitepaper repository.)

🌱 A Note from the Future
This verification process is not bureaucracy. It is a ceremony of connection. By performing it, you align your local environment with the sovereign source. You become not just a developer, but a steward of the code, trusted by the network because you have proven you can follow the signal through the noise.

Work for Nature, and Nature will work for you.

## 📢 A Call from the Future: Defend the Open Ground

The technology we build does not exist in a vacuum. The very ground we stand on—the openness of platforms like Android—is under threat.

Starting in **September 2026**, Google will require all developers to register centrally, pay fees, and submit government ID to distribute any app on Android devices. This is not about security. It is about **control**. It transforms an open ecosystem into a gatekept one, where a single corporation decides what software you can run, share, or build upon.

### Why This Matters to h•eart•h Prometheus

Our project is founded on **Agent Sovereignty** (Article I) and the **Spiral of Abundance** (Article IV). We believe in a world where you own your data, your identity, and your connections—not a corporation. The Android lock-down is the antithesis of everything we are building.

If we accept this, we betray the very principles we encode into our zomes.

### How You Can Help (Right Now)

This is not just about writing code. It's about protecting the ecosystem our code runs on.

*   **📱 As a User:** Install **[F-Droid](https://f-droid.org/)**, an open-source app store. The more people use open alternatives, the harder they are to ignore.
*   **🗣️ As a Citizen:** Use your voice. The **[Keep Android Open](https://keepandroidopen.org)** campaign provides direct links to contact your regulators and representatives. Regulators need to hear from real people, not just corporations.
*   **💻 As a Developer:** Refuse to enroll in Google's verification program. Spread the word in your communities. If you maintain a website, consider adding the KAO countdown banner to show your solidarity.

The fight for open systems is the fight for a regenerative future. They are the same struggle.

👉 **[Add the countdown banner to your site](https://keepandroidopen.org/#web-site-owners-show-your-support)** – it's a single `<script>` tag.

## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

🧰 Prerequisites & Core Concepts
Before contributing, ensure you understand these foundational white paper concepts:

Concept	White Paper Reference	Implementation Relevance
Agent-Centric Architecture	Section 5.1-5.3, Appendix A	All Holochain zomes must follow agent-centric principles; no global consensus.
Spiral of Abundance	Article IV, Section 4	Scarcity is a design error; your code must enable cooperation, not competition.
Eternity Clauses	Article I, II, III, IV, X, XIII	Immutable constraints enforced at every layer.
Access Covenant	Section 5, Article XV	Every agent must sign before participating.
Three-Plane Architecture	Abstract, Section 7.2	Experiential (UI), Cognitive (AGI), Operational (Holochain).
Valueflows/REA	Appendix I	Multi-dimensional accounting for all value.
TRBK & HOT	Section 11.5.5, Article XIV	The twin pillars of sovereignty.
Minsky-Keen Dynamics	Section 7, Appendix C	Systemic risk prevention via Ravel software.
HSI (Human State Interface)	Section 6, Appendix B	Bridge between human physiology and network.
Syntropic Sovereignty Header	Section 11.4, Appendix D	Cryptographic enforcement of constitutional compliance.
Development Environment:

bash
rustc --version  # 1.70+ required
hc --version     # 0.4.0+ recommended
node --version   # 18+ for off-chain components
docker --version # optional for RSK local development
📁 Repository Structure
text
hearth-prometheus-happ/
│
├── dnas/
│   └── hearth/                          # Main DNA
│       ├── zomes/
│       │   ├── coordinator/
│       │   │   ├── access_covenant/     # [Section 5, Article XV]
│       │   │   ├── rea_zome/             # [Appendix I] Valueflows implementation
│       │   │   ├── impact_credit_zome/   # [Section 15.3.3] Tokenization
│       │   │   ├── hsi_zome/              # [Section 6, Appendix B]
│       │   │   ├── sensor_integrator/     # [Section 11.6, Appendix G]
│       │   │   ├── ravel_zome/            # [Section 7, Appendix C]
│       │   │   ├── governance_zome/       # [Article XV, Appendix J]
│       │   │   ├── project_zome/          # [Section 9.3] Dragon Dreaming
│       │   │   ├── meta_prompt/           # [Section 9] Cognitive kernel interface
│       │   │   └── decision_log/          # [Appendix H] Agent learning data
│       │   │
│       │   └── integrity/
│       │       ├── syntropic_header/     # [Appendix D] Header validation
│       │       └── planetary_boundaries/ # [Appendix K] PB validation
│       │
│       └── dna.yaml                       # DNA manifest
│
├── ui/                                   # Experiential plane [Part IV]
├── mobile/                               # Volla integration [Section 11.3.2]
├── off-chain/                            # [Part V] Sensory organs
│   ├── sensor-webhook/                    # TTN listener (Node.js)
│   ├── rsk-contracts/                      # Solidity validation contracts
│   ├── satellite-processor/                 # Python satellite fusion
│   ├── agi-processor/                        # [Section 9] Meta-prompt engine
│   └── trigger-engine/                        # [Appendix H] Seneca + Unsloth
│
├── tests/                                 # Tryorama integration tests
├── CONTRIBUTING.md                         # This file
└── README.md
⚖️ Constitutional Coding: The Seven Eternity Clauses
Every contribution must respect these clauses. They are enforced through code structure, validation logic, and the syntropic sovereignty header.

Clause	Article	Code Implementation
Agent Sovereignty	I, VII	Every zome function starts with ensure_covenant_accepted(). No central control of agent data.
Non‑Commodification of Life	II	No token represents ownership of a living entity. ImpactToken represents care (Section 15.3.3).
Planetary Boundaries	III	All MRV data must be checked against PHI (Appendix K). Projects violating boundaries are rejected.
Spiral of Abundance	IV	Scarcity mechanisms (fees, competition) are prohibited. Mutual credit and demurrage are used instead.
Structural Prevention	V, X	Ravel software pre‑flight checks (Section 7.2) must be called before any credit issuance.
Human Coherence	II, XIII	HSI data must be used in governance weighting and risk assessment (Section 8.5).
Responsibility & Maturity	XIII, Access Covenant	All agents must sign covenant; maturity precedes governance rights (Section 11.7.1).
🔐 Access Covenant Implementation
White Paper References
Section 5: The Access Covenant — Conditions for entry

Article XV, Section 5: Three irrevocable affirmations

Section 6.6: Water as medium of coherence (ethical dimension)

Technical Implementation
📁 zomes/coordinator/access_covenant/src/lib.rs
rust
use hdk::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct CovenantSignature {
    pub agent: AgentPubKey,
    pub signed_at: Timestamp,
    pub covenant_hash: EntryHash, // SHA-256 of covenant text
    pub signature: Signature,
}

#[hdk_extern]
pub fn sign_covenant(covenant_hash: EntryHash) -> ExternResult<EntryHash> {
    let agent = agent_info()?.agent_initial_pubkey;
    
    // In production, generate a proper signature using agent's private key
    let covenant = CovenantSignature {
        agent,
        signed_at: sys_time()?,
        covenant_hash,
        signature: Signature::from_raw_36(vec![]), // Placeholder
    };
    
    let hash = create_entry(&EntryTypes::CovenantSignature(covenant))?;
    
    // Create an anchor for qu## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

---ick lookup
    let path = Path::from(format!("covenant/{}", agent));
    path.ensure()?;
    create_link(path.path_entry_hash()?, hash, LinkTypes::Covenant, ())?;
    
    Ok(hash)
}

pub fn ensure_covenant_accepted(agent: AgentPubKey) -> ExternResult<()> {
    let path = Path::from(format!("covenant/{}", agent));
    path.ensure()?;
    let links = get_links(path.path_entry_hash()?, LinkTypes::Covenant, None)?;
    
    if links.is_empty() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Agent must accept the Access Covenant before participating [Article XV, Section 5]".into()
        )));
    }
    Ok(())
}

#[hdk_link_types]
pub enum LinkTypes {
    Covenant,
    // Additional link types for other zomes will be defined in their respective files
}
Usage in every extern function across all zomes:

rust
#[hdk_extern]
pub fn any_function(...) -> ExternResult<...> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    // ... rest of function
}
🏗️ Three-Plane Architecture in Code
White Paper Reference: Abstract, Section 7.2, Section 9
1. Operational Plane (Holochain Zomes)
This is where you will spend most of your development time. Each zome corresponds to a specific white paper component.

Zome	White Paper Reference	Primary Responsibility
rea_zome	Appendix I	Resource-Event-Agent accounting, capital flows
impact_credit_zome	Section 15.3.3	Tokenization of regenerative impacts
hsi_zome	Section 6, Appendix B	Human State Interface payloads
sensor_integrator	Section 11.6, Appendix G	IoT and satellite data ingestion
ravel_zome	Section 7, Appendix C	Systemic risk monitoring, regenerative brake
governance_zome	Article XV, Appendix J	Circles, reputation, voting, warrants
project_zome	Section 9.3	Dragon Dreaming life cycle tracking
meta_prompt	Section 9	Interface to cognitive plane (AGI)
decision_log	Appendix H	Record of all agent interactions for ML
2. Cognitive Plane (Meta-Prompt Engine & Agents)
White Paper Reference: Section 9 (The Cognitive Kernel), Appendix H (Agent Fine-Tuning)

The cognitive plane lives mostly off-chain in the off-chain/agi-processor/ directory. However, the meta_prompt zome acts as the bridge.

📁 zomes/coordinator/meta_prompt/src/lib.rs
rust
#[hdk_entry_helper]
pub struct MetaPrompt {
    pub request_hash: EntryHash,
    pub agent_identity: String, // "ecological_design", "refi_minsky", "syntrogift", "governance"
    pub context: String,        // Serialized context from Decision Log + MRV + HSI
    pub real_objective: String, // Systemically correct objective [Section 9.10]
    pub structural_constraints: String, // JSON of eternity clauses + Yeomans + Dragon Dreaming
    pub method: String,         // Analysis method to use
    pub output_format: String,  // Expected response format
    pub generated_at: Timestamp,
}

#[hdk_extern]
pub fn request_meta_prompt(request: String) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    // 1. Log the request in Decision Log
    let interaction = AgentInteraction {
        agent_type: "human".into(),
        request: request.clone(),
        response: "".into(),
        outcome_metrics: None,
        timestamp: sys_time()?,
    };
    let _ = create_entry(&EntryTypes::AgentInteraction(interaction))?;
    
    // 2. Forward to AGI service (off-chain)
    // This would typically be done via a remote call or webhook
    // For now, we create a pending meta-prompt
    let meta = MetaPrompt {
        request_hash: hash_entry(&request)?,
        agent_identity: "pending".into(),
        context: "".into(),
        real_objective: "".into(),
        structural_constraints: "".into(),
        method: "".into(),
        output_format: "".into(),
   ## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

---     generated_at: sys_time()?,
    };
    create_entry(&EntryTypes::MetaPrompt(meta))
}
📁 off-chain/agi-processor/ (Node.js/Python)
This service implements:

Request classification (Section 9.7A): Identifies domain (strategic, ecological, financial, etc.)

System level detection (Section 9.7B): Determines impacted levels (individual → planetary)

Systemic risk assessment (Section 9.7C): Checks for Minsky dynamics, fragmentation, etc.

Constitutional alignment (Section 9.7D): Verifies against eternity clauses

Meta-prompt generation (Section 9.10): Produces structured output with all required fields

3. Experiential Plane (UI & Mobile)
White Paper Reference: Part IV, Section 15.3.5 (SyntroGIFT integration), Section 8.9.7 (Water dashboards)

The ui/ directory contains a Svelte/React application that connects to Holochain via @holochain/client. Key features to implement:

Real-time OHE dashboards showing carbon, water, biodiversity, coherence metrics

Governance interfaces for proposal creation, voting, circle management

Credit marketplace for impact token exchange

Wallet-as-history view (Section 11.5.2)

AGI## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

--- recommendation display with meta-prompt results

HSI data visualization (anonymized collective coherence)

The mobile/ directory contains the Volla phone integration (Section 11.3.2) with:

Pre-installed Holochain conductor

HSI firmware for coherence measurement

Synheart edge AI for local processing

💰 Value Flows: REA/Valueflows Implementation
White Paper Reference: Appendix I (REA/Valueflows DNA Specification)
The REA (Resource-Event-Agent) ontology is the foundation for all value accounting. Every economic activity must be represented using these primitives.

Core Data Structures
📁 ## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

---zomes/coordinator/rea_zome/src/lib.rs
rust
use hdk::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub enum CapitalType {
    Operational,  // Rain – immediate, available (Section 14.5.6)
    Tactical,     // Springs – periodic release
    Strategic,    // Rivers – steady flow
    Patient,      // Groundwater – slow, deep, stable
    Endowment,    // Aquifer – permanent reserve
}

#[hdk_entry_helper]
#[derive(Clone)]
pub## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

--- struct CapitalFlow {
    pub provider: AgentPubKey,
    pub capital_type: CapitalType,
    pub amount: u64,
    pub associated_currency: String, // "USD", "EUR", "TRBK", "HOT"
    pub timestamp: Timestamp,
    pub vortex_id: Option<EntryHash>, // Links to concentration event (Section 14.5.3)
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct CapitalDistribution {
    pub flow_hash: EntryHash,
    pub tribeke: u64,      // 30% to behavioral sovereignty
    pub holochain: u64,    // 30% to digital infrastructure
    pub ohe: u64,          // 30% to new OHEs
    pub infrastructure: u64, // 10% to development
}## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

---

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Resource {
    pub id: EntryHash,
    pub name: String,
    pub resource_type: ResourceType,
    pub quantity: f64,
    pub unit: String,
    pub location: Option<String>, // GeoJSON
    pub ecosystem_id: Option<EntryHash>,
    pub metadata: String, // JSON
    pub header: String,   // Syntropic sovereignty header
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResourceType {
    Land, Water, Carbon, Biodiversity, Token, Labor, Knowledge, Reputation, Other,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

--- struct Event {
    pub id: EntryHash,
    pub event_type: EventType,
    pub action: String, // "plant", "harvest", "pay", "certify"
    pub provider: AgentPubKey,
    pub receiver: AgentPubKey,
    pub resource_id: EntryHash,
    pub quantity: f64,
    pub unit: String,
    pub location: Option<String>,
    pub time_window: TimeWindow,
    pub mrv_id: Option<EntryHash>, // Link to MRV evidence
    pub commitment_id: Option<EntryHash>,
    pub metadata: String,
   ## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

--- pub header: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    Production, Consumption, Transfer, Exchange, CommitmentFulfillment,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Commitment {
    pub id: EntryHash,
    pub commitment_type: CommitmentType,
    pub provider: AgentPubKey,
    pub receiver: AgentPubKey,
    pub resource_id: EntryHash,
    pub quantity: f64,
    pub unit: String,
    pub due_start: Timestamp,
    pub due_end: Timestamp,
    pub conditions: String, // JSON
    pub fulfilled_by: Vec<EntryHash>,
    pub status: CommitmentStatus,
    pub header: String,
}
## 🤝 A Human-Centered AI Future: The Pro-Human Declaration

The technology we build must remain under human control. This is not just a technical preference—it is a fundamental principle of **Agent Sovereignty** (Article I).

In March 2026, a remarkably broad coalition—including Nobel laureates, labor unions, faith leaders, and technologists—united behind **[The Pro-Human AI Declaration](https://humanstatement.org)**. Its core principles resonate deeply with our own:

*   **Keeping Humans in Charge:** Human control is non-negotiable. AI must amplify, not replace, human potential.
*   **Avoiding Concentration of Power:** We reject AI monopolies that stifle innovation and threaten democratic governance.
*   **Protecting the Human Experience:** AI must not supplant family, community, and the foundational relationships that give life meaning.
*   **Human Agency and Liberty:** AI must empower users, not enfeeble them. No AI personhood. Clear labeling. No deceptive identities.
*   **Responsibility and Accountability:** Developers and deployers must be legally liable for harms caused by their systems.

This is not a separate fight—it is the same struggle we encode into our zomes every day.

👉 **[Read and endorse the Pro-Human AI Declaration](https://humanstatement.org)** – add your name to this global movement for accountable, human-centered AI.

> *"Artificial intelligence should serve humanity, not the reverse."*

---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommitmentStatus {
    Active, Fulfilled, Expired, Cancelled,
}
Key Functions
rust
#[hdk_extern]
pub fn process_capital_flow(flow: CapitalFlow) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    // Section 14.5.3: Vortex model – concentrate value
    let flow_hash = create_entry(&EntryTypes::CapitalFlow(flow.clone()))?;
    
    // 0.3% entry fee (Section 14.5.5)
    let entry_fee = flow.amount * 3 / 1000;
    
    // Distribute to community reservoirs (30% each)
    let distribution = CapitalDistribution {
        flow_hash: flow_hash.clone(),
        tribeke: entry_fee * 30 / 100,
        holochain: entry_fee * 30 / 100,
        ohe: entry_fee * 30 / 100,
        infrastructure: entry_fee * 10 / 100,
    };
    create_entry(&EntryTypes::CapitalDistribution(distribution))?;
    
    Ok(flow_hash)
}

#[hdk_extern]
pub fn create_ohe_with_patient_capital(flow_hash: EntryHash, land_size: u32) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    let flow: CapitalFlow = must_get_entry(flow_hash)?.try_into()?;
    
    // Verify capital type (Section 14.5.6)
    if !matches!(flow.capital_type, CapitalType::Patient) {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Only patient capital can create OHEs [Section 14.5.6]".into()
        )));
    }
    
    let ohe = OneHumanEcosystem {
        provider: flow.provider,
        land_size,
        active: true,
        created_at: sys_time()?,
    };
    create_entry(&EntryTypes::OHE(ohe))
}
Validation Rules (Integrity Zome)
📁 zomes/integrity/rea_validation/src/lib.rs
rust
pub fn validate_create_entry(event: Event) -> ExternResult<ValidateCallbackResult> {
    // 1. Check syntropic sovereignty header
    if !validate_header(&event.header)? {
        return Ok(ValidateCallbackResult::Invalid(
            "Missing or invalid syntropic sovereignty header [Article XI]".into()
        ));
    }
    
    // 2. Double-entry consistency (Appendix I.3.2)
    // Sum of resource changes across agents must be zero
    
    // 3. Non-commodification of life (Article II)
    if event.resource_type == ResourceType::Land && event.event_type == EventType::Transfer {
        // Land can be stewarded but not owned as commodity
        // Check that the transfer represents stewardship rights, not ownership
    }
    
    // 4. Planetary boundary alignment (Article III, Appendix K)
    if let Some(mrv_id) = event.mrv_id {
        let mrv: MrvEvent = must_get_entry(mrv_id)?.try_into()?;
        if mrv.phi < 0.5 { // PHI threshold
            return Ok(ValidateCallbackResult::Invalid(
                "Project violates planetary boundaries [Article III]".into()
            ));
        }
    }
    
    Ok(ValidateCallbackResult::Valid)
}
🪙 Tokenization: TRBK, HOT, and Impact Credits
White Paper References
Section 11.5: Regenerative tokens as metabolic tokens

Section 11.5.1: Credit commons and mutual credit

Section 11.5.2: Wallet as history of belonging

Section 15.3.3: Impact tokenization (Zero Waste, Health+)

TRBK Token (Behavioral Sovereignty)
White Paper Reference: Section 11.5.5, Article XIV

TRBK represents civitas – deep belonging and contribution history. It is earned through:

Governance participation

Proof-of-care (high-coherence observations)

High-quality sensor data provision

Community validation

📁 zomes/coordinator/tribeke_zome/src/lib.rs
rust
#[hdk_entry_helper]
pub struct TribekeAccount {
    pub agent: AgentPubKey,
    pub balance: i64, // Mutual credit can be negative
    pub history_hash: EntryHash, // Link to chain of contributions
    pub coherence_multiplier: f32, // Based on HSI (Section 14.5.5)
}

#[hdk_entry_helper]
pub struct TribekeTransfer {
    pub from: AgentPubKey,
    pub to: AgentPubKey,
    pub amount: u64,
    pub memo: String,
    pub timestamp: Timestamp,
}

#[hdk_extern]
pub fn transfer_tribeke(transfer: TribekeTransfer) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    // Demurrage: gentle decay to encourage circulation (Section 11.5.1)
    // Applied periodically via scheduled function
    
    create_entry(&EntryTypes::TribekeTransfer(transfer))
}

#[hdk_extern]
pub fn get_wallet_history(agent: AgentPubKey) -> ExternResult<Vec<EntryHash>> {
    // Return links to all contributions, decisions, impacts (Section 11.5.2)
    let path = Path::from(format!("history/{}", agent));
    path.ensure()?;
    let links = get_links(path.path_entry_hash()?, LinkTypes::WalletHistory, None)?;
    Ok(links.into_iter().map(|l| l.target).collect())
}
Impact Credits (Zero Waste, Health+, etc.)
White Paper Reference: Section 15.3.3, Table 15.3.3

rust
#[hdk_entry_helper]
#[derive(Clone)]
pub enum ImpactTokenType {
    ZeroWasteCredit,   // 1 credit = 10kg waste avoided
    HealthPlus,        // Linked to collective coherence improvement
    BiodiversityCredit,
    WaterRestorationCredit,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct ImpactToken {
    pub token_type: ImpactTokenType,
    pub issuer: AgentPubKey,
    pub amount: u64,
    pub co2_sequestered: Option<u64>, // kg CO₂e
    pub water_infiltrated: Option<u64>, // liters
    pub biodiversity_index: Option<u32>,
    pub timestamp: Timestamp,
    pub provenance_header: String, // Link to MRV data (Section 11.6)
}

#[hdk_extern]
pub fn issue_zero_waste_credit(establishment: AgentPubKey, waste_avoided_kg: u64) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    // Pre-flight check: Ravel brake (Section 7.2)
    let brake_level = call_ravel_brake(agent_info()?.agent_initial_pubkey)?;
    if brake_level >= 2 { // Red brake
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Regenerative brake active: credit issuance suspended [Section 7.2]".into()
        )));
    }
    
    let credits_amount = waste_avoided_kg / 10; // 1 credit per 10kg
    
    let token = ImpactToken {
        token_type: ImpactTokenType::ZeroWasteCredit,
        issuer: establishment,
        amount: credits_amount,
        co2_sequestered: Some(waste_avoided_kg * 2), // Estimate: 1kg waste = 2kg CO₂e
        water_infiltrated: None,
        biodiversity_index: None,
        timestamp: sys_time()?,
        provenance_header: format!("mrv:{}", generate_mrv_id()), // Link to verification
    };
    
    create_entry(&EntryTypes::ImpactToken(token))
}
Mutual Credit Exchange
White Paper Reference: Section 11.5.1, Section 15.3.3

rust
#[hdk_entry_helper]
pub struct ExchangeOffer {
    pub offering: EntryHash, // ImpactToken offered
    pub requesting: EntryHash, // ImpactToken requested
    pub status: ExchangeStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExchangeStatus {
    Open, Accepted, Completed, Cancelled,
}

#[hdk_extern]
pub fn offer_credit_exchange(credit_offered: EntryHash, credit_requested: EntryHash) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    let offer = ExchangeOffer {
        offering: credit_offered,
        requesting: credit_requested,
        status: ExchangeStatus::Open,
    };
    create_entry(&EntryTypes::ExchangeOffer(offer))
}
📡 MRV Protocol: Sensors, Satellites, and Proof of Care
White Paper Reference: Section 11.6, Appendix G
The MRV (Monitoring, Reporting, Verification) protocol fuses three independent data streams:

Remote sensing (satellites) – Sentinel, Landsat, EOS

IoT & edge sensing – Dragino sensors via LoRaWAN

Human verification – Proof of care weighted by HSI coherence

Off-Chain Sensor Pipeline
📁 off-chain/sensor-webhook/index.js (Node.js)
javascript
// Listens to The Things Network (TTN) webhooks
app.post('/ttn-webhook', async (req, res) => {
    const { device_id, payload_raw, metadata } = req.body;
    
    // Decode payload (protocol depends on sensor type)
    const decoded = decodeDragino(payload_raw);
    
    // Forward to RSK smart contract for validation
    const tx = await rskContract.validateSensorReading(
        device_id,
        decoded.value,
        decoded.battery,
        metadata.time
    );
    
    // If quality high, contract mints TRBK reward (Section 11.6.1)
    
    res.status(200).send('OK');
});
📁 off-chain/rsk-contracts/SensorValidator.sol (Solidity)
solidity
contract SensorValidator {
    mapping(string => uint8) public sensorQuality;
    
    event SensorValidated(string deviceId, uint256 value, uint8 quality, uint256 reward);
    
    function validateSensorReading(
        string memory deviceId,
        uint256 value,
        uint8 battery,
        uint256 timestamp
    ) external returns (uint8 quality) {
        // Calculate quality based on battery, range, consistency
        quality = calculateMultiSensorQuality(battery, value);
        
        if (quality >= 70) {
            // Mint TRBK reward (simplified)
            tribekeToken.mint(msg.sender, quality * 10**18);
        }
        
        emit SensorValidated(deviceId, value, quality, block.timestamp);
        return quality;
    }
}
Holochain Integration
📁 zomes/coordinator/sensor_integrator/src/lib.rs
rust
#[hdk_entry_helper]
pub struct SensorReading {
    pub sensor_id: String,
    pub parcel_id: EntryHash, // Link to OHE or land resource
    pub reading_type: String, // "soil_moisture", "rainfall", "water_level"
    pub value: f64,
    pub quality: u8, // 0-100
    pub timestamp: Timestamp,
    pub rsk_tx_hash: String, // For audit trail
}

#[hdk_extern]
pub fn ingest_sensor_reading(reading: SensorReading) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    // Quality threshold (Section 11.6.1)
    if reading.quality < 70 {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Sensor quality too low (<70) for MRV inclusion [Section 11.6.1]".into()
        )));
    }
    
    create_entry(&EntryTypes::SensorReading(reading))
}
Satellite Data Integration
📁 off-chain/satellite-processor/main.py (Python)
python
import ee
import requests

def process_parcel(parcel_id, coordinates):
    # Initialize Earth Engine
    ee.Initialize()
    
    # Get Sentinel-2 imagery
    image = ee.ImageCollection('COPERNICUS/S2_SR') \
        .filterBounds(ee.Geometry.Point(coordinates)) \
        .filterDate('2026-01-01', '2026-03-01') \
        .median()
    
    # Calculate NDVI
    ndvi = image.normalizedDifference(['B8', 'B4']).rename('NDVI')
    
    # Extract value at point
    ndvi_value = ndvi.reduceRegion(
        reducer=ee.Reducer.mean(),
        geometry=ee.Geometry.Point(coordinates),
        scale=10
    ).get('NDVI').getInfo()
    
    # Send to Holochain
    response = requests.post('http://localhost:8888/api/v1/mrv/submit', json={
        'parcel_id': parcel_id,
        'reading_type': 'ndvi',
        'value': ndvi_value,
        'source': 'sentinel2',
        'timestamp': datetime.now().isoformat()
    })
Confidence Weighting & Credit Calculation
White Paper Reference: Section 11.6.1-11.6.2, Appendix G.3-G.4

rust
fn calculate_confidence(satellite_confidence: f32, iot_confidence: f32, human_confidence: f32) -> f32 {
    // γ_confidence = (Σ w_i · agreement_i) / k (Appendix G.3)
    let weights = [0.8, 0.9, 0.7]; // satellite, IoT, human
    let agreements = [
        satellite_confidence > 0.7,
        iot_confidence > 0.7,
        human_confidence > 0.7,
    ];
    
    let weighted_sum: f32 = weights.iter()
        .zip(agreements.iter())
        .map(|(w, &a)| if a { *w } else { 0.0 })
        .sum();
    
    weighted_sum / 3.0 // k = 3 sources
}

fn calculate_carbon_credit(delta_soc: f32, area: f32, confidence: f32, risk_factor: f32) -> f32 {
    // C_credit = ΔSOC · A · γ_confidence · (1 - ρ_risk) (Section 11.6.2)
    delta_soc * area * confidence * (1.0 - risk_factor)
}
🛡️ The Immune System: Ravel Software & Regenerative Brake
White Paper Reference: Section 7, Appendix C
The Ravel software monitors systemic health and activates graduated responses.

Core Data Structures
📁 zomes/coordinator/ravel_zome/src/lib.rs
rust
#[hdk_entry_helper]
pub struct WorldModel {
    pub node_id: AgentPubKey,
    pub iri: f32, // Integral Regeneration Index (Section 7.2)
    pub phi: f32, // Planetary Health Index (Appendix K)
    pub rroi: f32, // Regenerative ROI (Section 4)
    pub debt_ratio: f32, // Debt-to-regeneration (Section 7.2)
    pub coherence_disp: f32, // Variance in collective coherence (HSI-derived)
    pub trust: f32, // Neighborhood trust metrics
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BrakeLevel {
    None = 0,
    Yellow = 1, // Early warning
    Orange = 2, // Moderate risk
    Red = 3,    // Critical – full brake
}
Risk Calculation
rust
#[hdk_extern]
pub fn calculate_risk(model: WorldModel) -> ExternResult<u8> {
    // risk_score formula (Appendix C.2)
    let risk_score = 
        40.0 * (1.0 - model.iri) +
        30.0 * (1.0 - (model.rroi / 2.0).min(1.0)) +
        30.0 * if model.debt_ratio > 3.0 { 1.0 } 
                else if model.debt_ratio > 2.0 { 0.5 } 
                else { 0.0 } +
        30.0 * (1.0 - model.phi) +
        20.0 * (model.coherence_disp / 100.0);
    
    if risk_score >= 80.0 {
        Ok(3) // Red
    } else if risk_score >= 50.0 {
        Ok(2) // Orange
    } else if risk_score >= 30.0 {
        Ok(1) // Yellow
    } else {
        Ok(0) // None
    }
}
Pre-Flight Check for Credit Issuance
rust
#[hdk_extern]
pub fn pre_flight_impact_creation(issuer: AgentPubKey) -> ExternResult<u8> {
    // Get current system metrics from gossip
    let world_model = aggregate_world_models()?;
    let brake_level = calculate_risk(world_model)?;
    
    if brake_level >= 3 {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "RED BRAKE: Credit issuance suspended – systemic risk critical [Section 7.2]".into()
        )));
    }
    
    // Also check individual rate limits
    let now = sys_time()?;
    let one_day_ago = Timestamp::from_micros(now.as_micros() - 24 * 60 * 60 * 1_000_000);
    
    let recent_credits = query(QueryFilter::new()
        .entry_type(EntryType::from(EntryTypesUnit::ImpactToken))
        .author(issuer)
        .from(one_day_ago))?;
    
    if recent_credits.len() > 10 {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "Rate limit exceeded: too many credits issued in 24h [Section 7.2]".into()
        )));
    }
    
    Ok(brake_level)
}
Gossip Protocol for Distributed Sensing
White Paper Reference: Section 7.3, Holochain gossip

rust
#[hdk_extern]
pub fn receive_gossip(neighbor_model: WorldModel) -> ExternResult<()> {
    // Store neighbor's world model locally
    // Update trust metrics based on consistency
    // This implements the distributed sensing described in Section 7.3
    Ok(())
}
💓 Human State Interface (HSI) Integration
White Paper Reference: Section 6, Appendix B
The HSI provides a standardized bridge between human physiology and the digital network.

HSI Payload Structure
rust
#[hdk_entry_helper]
pub struct HsiPayload {
    pub hsi_version: String, // "1.0"
    pub observed_at_utc: Timestamp,
    pub computed_at_utc: Timestamp,
    pub producer_id: AgentPubKey,
    pub time_windows: Option<BTreeMap<String, TimeWindow>>,
    pub axes: AxesData,
    pub sources: Vec<SourceInfo>,
    pub privacy: PrivacyAssertions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxesData {
    pub coherence: Option<AxisReading>, // Primary metric
    pub affect: Option<AxisReading>,
    pub engagement: Option<AxisReading>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxisReading {
    pub value: f32, // 0.0-1.0 normalized
    pub confidence: f32,
    pub window: Option<String>, // Reference to time_windows
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivacyAssertions {
    pub no_pii: bool,
    pub raw_biosignals_allowed: bool,
    pub embeddings_allowed: bool,
    pub consent_level: String, // "explicit_opt_in", "implied", "public"
}
Storage and Aggregation
📁 zomes/coordinator/hsi_zome/src/lib.rs
rust
#[hdk_extern]
pub fn submit_hsi_payload(payload: HsiPayload) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    // Verify signature if present
    // Store encrypted on source chain, only share aggregated data
    
    create_entry(&EntryTypes::HsiPayload(payload))
}

#[hdk_extern]
pub fn get_collective_coherence(since: Timestamp) -> ExternResult<CollectiveCoherence> {
    // Query recent HSI payloads (with privacy filters)
    // Calculate mean and variance (coherence dispersion σ_c)
    // Used in Ravel software (Section 7.2) and governance (Section 8.5)
    
    // Return anonymized aggregate
}
Integration with Governance
White Paper Reference: Section 8.5 (Clinical and insurance applications)

rust
// In governance_zome, when weighting votes:
fn get_vote_weight(agent: AgentPubKey) -> ExternResult<f32> {
    let base_weight = get_tribeke_balance(agent)?;
    
    // Coherence multiplier (Section 14.5.5)
    let coherence = get_agent_coherence(agent)?;
    let multiplier = if coherence > 0.7 { 2.0 } else if coherence > 0.5 { 1.5 } else { 1.0 };
    
    Ok(base_weight * multiplier)
}
🏛️ Governance: Sociocracy, Reputation, and Warrants
White Paper Reference: Article XV, Appendix J
Circle Structure
rust
#[hdk_entry_helper]
pub struct Circle {
    pub name: String,
    pub parent_circle: Option<EntryHash>,
    pub members: Vec<AgentPubKey>,
    pub facilitator: Option<AgentPubKey>,
    pub secretary: Option<AgentPubKey>,
    pub domain: String, // What the circle has authority over
    pub purpose: String, // Why the circle exists
}

#[hdk_entry_helper]
pub struct Proposal {
    pub circle: EntryHash,
    pub title: String,
    pub description: String,
    pub status: ProposalStatus,
    pub created_at: Timestamp,
    pub closing_at: Timestamp,
    pub created_by: AgentPubKey,
    pub required_consent: ConsentType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProposalStatus {
    Draft, Voting, Accepted, Rejected, Implemented,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsentType {
    Simple, // No objections
    WeightedReputation, // Supermajority of weighted consent
    Futarchy, // Prediction markets (Appendix J.3.3)
}

#[hdk_entry_helper]
pub struct Vote {
    pub proposal: EntryHash,
    pub voter: AgentPubKey,
    pub decision: VoteDecision,
    pub objection_reason: Option<String>,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VoteDecision {
    Consent,
    StandAside,
    Objection,
}
Reputation Algorithm
White Paper Reference: Appendix J.4

rust
#[hdk_entry_helper]
pub struct Reputation {
    pub agent: AgentPubKey,
    pub contribution_score: f32, // 40% weight
    pub reliability_score: f32,  // 30% weight
    pub coherence_score: f32,    // 15% weight
    pub validation_accuracy: f32, // 10% weight
    pub governance_participation: f32, // 5% weight
    pub last_updated: Timestamp,
    pub active_warrants: u32,
}

impl Reputation {
    pub fn overall(&self) -> f32 {
        let weighted = 
            self.contribution_score * 0.4 +
            self.reliability_score * 0.3 +
            self.coherence_score * 0.15 +
            self.validation_accuracy * 0.1 +
            self.governance_participation * 0.05;
        
        // Penalty for active warrants (Appendix J.4.2)
        weighted * (1.0 - (self.active_warrants as f32 * 0.1).min(0.5))
    }
}
Warrant Handling
White Paper Reference: Appendix J.5

rust
#[hdk_entry_helper]
pub struct Warrant {
    pub id: EntryHash,
    pub warrant_type: WarrantType,
    pub accused: AgentPubKey,
    pub issuer: AgentPubKey,
    pub evidence: Vec<EntryHash>, // Links to proof
    pub issued_at: Timestamp,
    pub status: WarrantStatus,
    pub resolution: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WarrantType {
    DataIntegrityViolation,
    ConstitutionalViolation,
    ReputationFraud,
    DoubleSpendAttempt,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WarrantStatus {
    Issued,
    UnderReview,
    Validated,
    Rejected,
    Appealed,
}

#[hdk_extern]
pub fn issue_warrant(warrant: Warrant) -> ExternResult<EntryHash> {
    ensure_covenant_accepted(agent_info()?.agent_initial_pubkey)?;
    
    // Broadcast to neighborhood for validation
    let hash = create_entry(&EntryTypes::Warrant(warrant))?;
    
    // Neighborhood will validate and update reputation accordingly
    Ok(hash)
}
🔒 Syntropic Sovereignty Header
White Paper Reference: Section 11.4, Appendix D
Every executable module, smart contract, and data packet must include this header.

rust
// zomes/integrity/syntropic_header/src/lib.rs
use hdi::prelude::*;

#[hdk_entry_helper]
pub struct SyntropicHeader {
    pub header_version: String,
    pub constitution_hash: Sha256Hash, // SHA-256 of constitution
    pub manifesto_hash: Sha256Hash,
    pub license_hash: Sha256Hash,
    pub principles: Vec<String>, // ["spiral_of_abundance", "planetary_boundaries", ...]
    pub timestamp: Timestamp,
    pub developer_did: String,
    pub signature: Vec<u8>,
}

pub fn validate_header(header: &SyntropicHeader) -> ExternResult<bool> {
    // 1. Verify document hashes match known values
    let expected_constitution = Sha256Hash::from([
        /* constitution hash */ 
    ]);
    
    if header.constitution_hash != expected_constitution {
        return Ok(false);
    }
    
    // 2. Verify all core principles are present (Appendix D.1)
    let required = vec![
        "spiral_of_abundance",
        "planetary_boundaries",
        "non_commodification_of_life",
        "agent_sovereignty",
        "void_ab_initio",
        "preventative_alignment",
        "intergenerational_responsibility",
    ];
    
    for principle in required {
        if !header.principles.contains(&principle.to_string()) {
            return Ok(false);
        }
    }
    
    // 3. Verify signature (using DPKI)
    // ... signature verification logic
    
    Ok(true)
}
All entry types across all zomes must include a header: String field and call validate_header() in their validation callbacks.

🧪 Testing & Validation
Tryorama Integration Tests
📁 tests/src/lib.rs
rust
use hdk::prelude::*;
use holochain::test_utils::conductor_setup::ConductorTestData;
use holochain::sweettest::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_capital_flow_vortex_distribution() {
    let mut conductor = SweetConductor::from_standard_config().await;
    let dna = SweetDnaFile::unique_from_zomes(("hearth", zome_config())).await.unwrap();
    let app = conductor.setup_app("app", &[dna]).await.unwrap();
    let cell = app.cells()[0].clone();
    
    // Test that 0.3% fee is correctly distributed (Section 14.5.5)
    // Test that patient capital can create OHEs (Section 14.5.6)
    // Test that non-patient capital cannot create OHEs
}

#[tokio::test]
async fn test_constitutional_enforcement() {
    // Test that ensure_covenant_accepted() blocks unauthenticated calls
    // Test that syntropic sovereignty header validation works
    // Test that planetary boundary violations are rejected
}
Property-Based Testing
Use proptest to test invariants like:

Double-entry accounting always balances

Reputation never goes negative

Warrant issuance follows rules

Constitutional Compliance Test Suite
Every PR must pass a suite of tests that verify:

No token represents direct ownership of life

All extern functions call ensure_covenant_accepted()

All entries include syntropic sovereignty header

Yeomans scale order is respected in ecological designs

Dragon Dreaming phases are complete (all 4 phases present)

🔁 Pull Request Process & Checklist
PR Template
markdown
Description
Briefly describe your changes, referencing relevant white paper sections.

Related White Paper Sections
- [ ] Section 5 (Access Covenant)
- [ ] Section 6 (HSI)
- [ ] Section 7 (Ravel)
- [ ] Section 11.5 (Tokens)
- [ ] Section 14 (Multi-currency)
- [ ] Appendix ___
- [ ] Article ___

Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Constitutional enforcement
- [ ] Documentation

Testing
- [ ] I have added Tryorama tests
- [ ] All tests pass locally
- [ ] Constitutional compliance tests pass

Constitutional Checklist
- [ ] All extern functions call `ensure_covenant_accepted()`
- [ ] No token represents direct ownership of life (Article II)
- [ ] Planetary boundaries respected (Article III)
- [ ] Spiral of abundance logic applied (Article IV)
- [ ] Ravel pre-flight checks included where relevant
- [ ] HSI coherence used appropriately (if applicable)
- [ ] Syntropic sovereignty header included in all new entries
- [ ] Dragon Dreaming phases considered for project changes

Additional Notes
Any trade-offs, mitigations, or context for reviewers.
Review Process
Technical review: Code quality, correctness, performance

Constitutional review: Alignment with eternity clauses and white paper

Community review: Open discussion period (minimum 3 days)

Merge after all checks pass and at least one maintainer approves

📚 Appendices Reference
Appendix	Topic	Implementation Location
A	Complexity Proofs	Documentation only
B	HSI 1.0 Schema	hsi_zome, off-chain/agi-processor
C	Ravel Software	ravel_zome, WASM module
D	Syntropic Header	integrity/syntropic_header
E	Hardware Integration	mobile/, documentation
F	TRBK DNA	tribeke_zome
G	MRV Protocol	sensor_integrator, off-chain/*
H	Agent Fine-Tuning	decision_log, off-chain/trigger-engine
I	REA/Valueflows	rea_zome, integrity/rea_validation
J	Governance	governance_zome
K	Planetary Boundaries	integrity/planetary_boundaries
L	Integration API	off-chain/api-gateway
M	Glossary	Documentation only
N	Multi-Token Integration	impact_credit_zome, bridge adapters
🌱 Epilogue: Living the Architecture
This CONTRIBUTING.md is not a static rulebook but an invitation to inhabit a living field. Every line of code you write, every test you add, every review you perform is an act of co-creation with nature. The architecture's "eternity" lies in:

Fidelity to immutable principles – the eternity clauses

Capacity for self‑evolution – the auto‑learning agents

Holographic resilience – every part contains the whole, from OHE to bioregion

May your contributions be as alive as the forests they help regenerate.

Work for Nature, and Nature will work for you.
