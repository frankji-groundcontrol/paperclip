use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, OptionalExtension, Row};
use uuid::Uuid;

use crate::activity::{ActivityEvent, ActivityFilters, ActivityRepository, CreateActivity};
use crate::adapters::{AdapterRepository, CompanyAdapter, ConfigureAdapter, UpdateAdapter};
use crate::agents::{Agent, AgentRepository, CreateAgent, UpdateAgent};
use crate::approvals::{Approval, ApprovalRepository, CreateApproval, UpdateApproval};
use crate::companies::{Company, CompanyRepository, CreateCompany, UpdateCompany};
use crate::costs::{CostEvent, CostRepository, CreateCostEvent};
use crate::environments::{
    CreateEnvironment, Environment, EnvironmentRepository, UpdateEnvironment,
};
use crate::goals::{CreateGoal, Goal, GoalRepository, UpdateGoal};
use crate::inbox::{InboxDismissal, InboxRepository};
use crate::issues::{CreateIssue, Issue, IssueRepository, UpdateIssue};
use crate::pipelines::{CreatePipeline, Pipeline, PipelineRepository, UpdatePipeline};
use crate::plugins::{CompanyPlugin, InstallPlugin, PluginRepository, UpdatePlugin};
use crate::projects::{CreateProject, Project, ProjectRepository, UpdateProject};
use crate::routines::{CreateRoutine, Routine, RoutineRepository, UpdateRoutine};
use crate::runs::{CreateRun, Run, RunRepository, UpdateRun};
use crate::secrets::{CreateSecret, SecretRef, SecretRepository};
use crate::workspaces::{CreateWorkspace, UpdateWorkspace, Workspace, WorkspaceRepository};

/// A SQLite-backed [`CompanyRepository`] — durable persistence in-process (no
/// external server), the migration analog of the production Postgres layer.
///
/// `Connection` is not `Sync`, so it is guarded by a `Mutex`; the `Arc` makes the
/// store cheap to clone and share (e.g. as axum state).
#[derive(Clone)]
pub struct SqliteCompanyStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCompanyStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS companies (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                budget_monthly_cents INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .expect("create companies table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

fn row_to_company(row: &Row) -> rusqlite::Result<Company> {
    Ok(Company {
        id: row.get(0)?,
        name: row.get(1)?,
        description: row.get(2)?,
        budget_monthly_cents: row.get(3)?,
    })
}

impl CompanyRepository for SqliteCompanyStore {
    fn create(&self, input: CreateCompany) -> Company {
        let company = Company {
            id: Uuid::new_v4().to_string(),
            name: input.name,
            description: input.description,
            budget_monthly_cents: input.budget_monthly_cents,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO companies (id, name, description, budget_monthly_cents)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    company.id,
                    company.name,
                    company.description,
                    company.budget_monthly_cents
                ],
            )
            .expect("insert company");
        company
    }

    fn list(&self) -> Vec<Company> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, name, description, budget_monthly_cents FROM companies ORDER BY name",
            )
            .expect("prepare list");
        let rows = stmt.query_map([], row_to_company).expect("query list");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect companies")
    }

    fn get(&self, id: &str) -> Option<Company> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, description, budget_monthly_cents FROM companies WHERE id = ?1",
            params![id],
            row_to_company,
        )
        .optional()
        .expect("query company by id")
    }

    fn update(&self, id: &str, patch: UpdateCompany) -> Option<Company> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, name, description, budget_monthly_cents FROM companies WHERE id = ?1",
                params![id],
                row_to_company,
            )
            .optional()
            .expect("query company");
        let mut company = current?;
        if let Some(name) = patch.name {
            company.name = name;
        }
        if let Some(description) = patch.description {
            company.description = Some(description);
        }
        if let Some(budget) = patch.budget_monthly_cents {
            company.budget_monthly_cents = budget;
        }
        conn.execute(
            "UPDATE companies SET name = ?1, description = ?2, budget_monthly_cents = ?3
             WHERE id = ?4",
            params![
                company.name,
                company.description,
                company.budget_monthly_cents,
                id
            ],
        )
        .expect("update company");
        Some(company)
    }

    fn delete(&self, id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute("DELETE FROM companies WHERE id = ?1", params![id])
            .expect("delete company");
        affected > 0
    }
}

/// A SQLite-backed [`IssueRepository`] — durable, company-scoped issue storage.
#[derive(Clone)]
pub struct SqliteIssueStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteIssueStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS issues (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'backlog'
            )",
            [],
        )
        .expect("create issues table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl IssueRepository for SqliteIssueStore {
    fn create(&self, company_id: &str, input: CreateIssue) -> Issue {
        let issue = Issue {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            title: input.title,
            status: input.status,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO issues (id, company_id, title, status)
                 VALUES (?1, ?2, ?3, ?4)",
                params![issue.id, issue.company_id, issue.title, issue.status],
            )
            .expect("insert issue");
        issue
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Issue> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, title, status FROM issues
                 WHERE company_id = ?1 ORDER BY title",
            )
            .expect("prepare list issues");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(Issue {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    title: row.get(2)?,
                    status: row.get(3)?,
                })
            })
            .expect("query issues");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect issues")
    }

    fn update(&self, company_id: &str, issue_id: &str, patch: UpdateIssue) -> Option<Issue> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, title, status FROM issues
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, issue_id],
                |row| {
                    Ok(Issue {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        title: row.get(2)?,
                        status: row.get(3)?,
                    })
                },
            )
            .optional()
            .expect("query issue");
        let mut issue = current?;
        if let Some(title) = patch.title {
            issue.title = title;
        }
        if let Some(status) = patch.status {
            issue.status = status;
        }
        conn.execute(
            "UPDATE issues SET title = ?1, status = ?2 WHERE company_id = ?3 AND id = ?4",
            params![issue.title, issue.status, company_id, issue_id],
        )
        .expect("update issue");
        Some(issue)
    }

    fn delete(&self, company_id: &str, issue_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM issues WHERE company_id = ?1 AND id = ?2",
                params![company_id, issue_id],
            )
            .expect("delete issue");
        affected > 0
    }
}

/// A SQLite-backed [`ProjectRepository`] — durable, company-scoped project storage.
#[derive(Clone)]
pub struct SqliteProjectStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteProjectStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                name TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'backlog'
            )",
            [],
        )
        .expect("create projects table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl ProjectRepository for SqliteProjectStore {
    fn create(&self, company_id: &str, input: CreateProject) -> Project {
        let project = Project {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            name: input.name,
            status: input.status,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO projects (id, company_id, name, status)
                 VALUES (?1, ?2, ?3, ?4)",
                params![project.id, project.company_id, project.name, project.status],
            )
            .expect("insert project");
        project
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Project> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, name, status FROM projects
                 WHERE company_id = ?1 ORDER BY name",
            )
            .expect("prepare list projects");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    name: row.get(2)?,
                    status: row.get(3)?,
                })
            })
            .expect("query projects");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect projects")
    }

    fn update(&self, company_id: &str, project_id: &str, patch: UpdateProject) -> Option<Project> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, name, status FROM projects
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, project_id],
                |row| {
                    Ok(Project {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        name: row.get(2)?,
                        status: row.get(3)?,
                    })
                },
            )
            .optional()
            .expect("query project");
        let mut project = current?;
        if let Some(name) = patch.name {
            project.name = name;
        }
        if let Some(status) = patch.status {
            project.status = status;
        }
        conn.execute(
            "UPDATE projects SET name = ?1, status = ?2 WHERE company_id = ?3 AND id = ?4",
            params![project.name, project.status, company_id, project_id],
        )
        .expect("update project");
        Some(project)
    }

    fn delete(&self, company_id: &str, project_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM projects WHERE company_id = ?1 AND id = ?2",
                params![company_id, project_id],
            )
            .expect("delete project");
        affected > 0
    }
}

/// A SQLite-backed [`AgentRepository`] — durable, company-scoped agent storage.
#[derive(Clone)]
pub struct SqliteAgentStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteAgentStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agents (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                name TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'general',
                status TEXT NOT NULL DEFAULT 'idle',
                adapter_type TEXT NOT NULL DEFAULT 'process'
            )",
            [],
        )
        .expect("create agents table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl AgentRepository for SqliteAgentStore {
    fn create(&self, company_id: &str, input: CreateAgent) -> Agent {
        let agent = Agent {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            name: input.name,
            role: input.role,
            status: "idle".to_string(),
            adapter_type: input.adapter_type,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO agents (id, company_id, name, role, status, adapter_type)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    agent.id,
                    agent.company_id,
                    agent.name,
                    agent.role,
                    agent.status,
                    agent.adapter_type
                ],
            )
            .expect("insert agent");
        agent
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Agent> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, name, role, status, adapter_type FROM agents
                 WHERE company_id = ?1 ORDER BY name",
            )
            .expect("prepare list agents");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(Agent {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    name: row.get(2)?,
                    role: row.get(3)?,
                    status: row.get(4)?,
                    adapter_type: row.get(5)?,
                })
            })
            .expect("query agents");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect agents")
    }

    fn update(&self, company_id: &str, agent_id: &str, patch: UpdateAgent) -> Option<Agent> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, name, role, status, adapter_type FROM agents
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, agent_id],
                |row| {
                    Ok(Agent {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        name: row.get(2)?,
                        role: row.get(3)?,
                        status: row.get(4)?,
                        adapter_type: row.get(5)?,
                    })
                },
            )
            .optional()
            .expect("query agent");
        let mut agent = current?;
        if let Some(name) = patch.name {
            agent.name = name;
        }
        if let Some(role) = patch.role {
            agent.role = role;
        }
        if let Some(status) = patch.status {
            agent.status = status;
        }
        if let Some(adapter_type) = patch.adapter_type {
            agent.adapter_type = adapter_type;
        }
        conn.execute(
            "UPDATE agents SET name = ?1, role = ?2, status = ?3, adapter_type = ?4
             WHERE company_id = ?5 AND id = ?6",
            params![
                agent.name,
                agent.role,
                agent.status,
                agent.adapter_type,
                company_id,
                agent_id
            ],
        )
        .expect("update agent");
        Some(agent)
    }

    fn delete(&self, company_id: &str, agent_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM agents WHERE company_id = ?1 AND id = ?2",
                params![company_id, agent_id],
            )
            .expect("delete agent");
        affected > 0
    }
}

/// A SQLite-backed [`RunRepository`] — durable, company-scoped run storage.
#[derive(Clone)]
pub struct SqliteRunStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteRunStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS runs (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'queued'
            )",
            [],
        )
        .expect("create runs table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl RunRepository for SqliteRunStore {
    fn create(&self, company_id: &str, input: CreateRun) -> Run {
        let run = Run {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            agent_id: input.agent_id,
            status: input.status,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO runs (id, company_id, agent_id, status)
                 VALUES (?1, ?2, ?3, ?4)",
                params![run.id, run.company_id, run.agent_id, run.status],
            )
            .expect("insert run");
        run
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Run> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, agent_id, status FROM runs
                 WHERE company_id = ?1 ORDER BY id",
            )
            .expect("prepare list runs");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(Run {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    agent_id: row.get(2)?,
                    status: row.get(3)?,
                })
            })
            .expect("query runs");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect runs")
    }

    fn update(&self, company_id: &str, run_id: &str, patch: UpdateRun) -> Option<Run> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, agent_id, status FROM runs
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, run_id],
                |row| {
                    Ok(Run {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        agent_id: row.get(2)?,
                        status: row.get(3)?,
                    })
                },
            )
            .optional()
            .expect("query run");
        let mut run = current?;
        if let Some(status) = patch.status {
            run.status = status;
        }
        if let Some(agent_id) = patch.agent_id {
            run.agent_id = agent_id;
        }
        conn.execute(
            "UPDATE runs SET agent_id = ?1, status = ?2 WHERE company_id = ?3 AND id = ?4",
            params![run.agent_id, run.status, company_id, run_id],
        )
        .expect("update run");
        Some(run)
    }

    fn delete(&self, company_id: &str, run_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM runs WHERE company_id = ?1 AND id = ?2",
                params![company_id, run_id],
            )
            .expect("delete run");
        affected > 0
    }
}

/// A SQLite-backed [`ApprovalRepository`] — durable, company-scoped approvals.
#[derive(Clone)]
pub struct SqliteApprovalStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteApprovalStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS approvals (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                issue_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending'
            )",
            [],
        )
        .expect("create approvals table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl ApprovalRepository for SqliteApprovalStore {
    fn create(&self, company_id: &str, input: CreateApproval) -> Approval {
        let approval = Approval {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            issue_id: input.issue_id,
            status: input.status,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO approvals (id, company_id, issue_id, status)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    approval.id,
                    approval.company_id,
                    approval.issue_id,
                    approval.status
                ],
            )
            .expect("insert approval");
        approval
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Approval> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, issue_id, status FROM approvals
                 WHERE company_id = ?1 ORDER BY id",
            )
            .expect("prepare list approvals");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(Approval {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    issue_id: row.get(2)?,
                    status: row.get(3)?,
                })
            })
            .expect("query approvals");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect approvals")
    }

    fn update(
        &self,
        company_id: &str,
        approval_id: &str,
        patch: UpdateApproval,
    ) -> Option<Approval> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, issue_id, status FROM approvals
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, approval_id],
                |row| {
                    Ok(Approval {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        issue_id: row.get(2)?,
                        status: row.get(3)?,
                    })
                },
            )
            .optional()
            .expect("query approval");
        let mut approval = current?;
        if let Some(status) = patch.status {
            approval.status = status;
        }
        if let Some(issue_id) = patch.issue_id {
            approval.issue_id = issue_id;
        }
        conn.execute(
            "UPDATE approvals SET issue_id = ?1, status = ?2 WHERE company_id = ?3 AND id = ?4",
            params![approval.issue_id, approval.status, company_id, approval_id],
        )
        .expect("update approval");
        Some(approval)
    }

    fn delete(&self, company_id: &str, approval_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM approvals WHERE company_id = ?1 AND id = ?2",
                params![company_id, approval_id],
            )
            .expect("delete approval");
        affected > 0
    }
}

/// A SQLite-backed [`RoutineRepository`] — durable, company-scoped routines.
#[derive(Clone)]
pub struct SqliteRoutineStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteRoutineStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS routines (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active'
            )",
            [],
        )
        .expect("create routines table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl RoutineRepository for SqliteRoutineStore {
    fn create(&self, company_id: &str, input: CreateRoutine) -> Routine {
        let routine = Routine {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            title: input.title,
            status: input.status,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO routines (id, company_id, title, status)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    routine.id,
                    routine.company_id,
                    routine.title,
                    routine.status
                ],
            )
            .expect("insert routine");
        routine
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Routine> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, title, status FROM routines
                 WHERE company_id = ?1 ORDER BY title",
            )
            .expect("prepare list routines");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(Routine {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    title: row.get(2)?,
                    status: row.get(3)?,
                })
            })
            .expect("query routines");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect routines")
    }

    fn update(&self, company_id: &str, routine_id: &str, patch: UpdateRoutine) -> Option<Routine> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, title, status FROM routines
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, routine_id],
                |row| {
                    Ok(Routine {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        title: row.get(2)?,
                        status: row.get(3)?,
                    })
                },
            )
            .optional()
            .expect("query routine");
        let mut routine = current?;
        if let Some(title) = patch.title {
            routine.title = title;
        }
        if let Some(status) = patch.status {
            routine.status = status;
        }
        conn.execute(
            "UPDATE routines SET title = ?1, status = ?2 WHERE company_id = ?3 AND id = ?4",
            params![routine.title, routine.status, company_id, routine_id],
        )
        .expect("update routine");
        Some(routine)
    }

    fn delete(&self, company_id: &str, routine_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM routines WHERE company_id = ?1 AND id = ?2",
                params![company_id, routine_id],
            )
            .expect("delete routine");
        affected > 0
    }
}

/// A SQLite-backed [`SecretRepository`]. The plaintext value is stored in its own
/// column but is NEVER selected back out — `list_by_company` reads only the
/// reference columns, preserving the no-leak invariant at the persistence layer.
#[derive(Clone)]
pub struct SqliteSecretStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteSecretStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                name TEXT NOT NULL,
                provider TEXT NOT NULL DEFAULT 'local',
                value TEXT NOT NULL
            )",
            [],
        )
        .expect("create secrets table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl SecretRepository for SqliteSecretStore {
    fn create(&self, company_id: &str, input: CreateSecret) -> SecretRef {
        let reference = SecretRef {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            name: input.name,
            provider: input.provider,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO secrets (id, company_id, name, provider, value)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    reference.id,
                    reference.company_id,
                    reference.name,
                    reference.provider,
                    input.value
                ],
            )
            .expect("insert secret");
        reference
    }

    fn list_by_company(&self, company_id: &str) -> Vec<SecretRef> {
        let conn = self.conn.lock().unwrap();
        // Note: the `value` column is intentionally never selected.
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, name, provider FROM secrets
                 WHERE company_id = ?1 ORDER BY name",
            )
            .expect("prepare list secrets");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(SecretRef {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    name: row.get(2)?,
                    provider: row.get(3)?,
                })
            })
            .expect("query secrets");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect secrets")
    }

    fn delete(&self, company_id: &str, secret_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM secrets WHERE company_id = ?1 AND id = ?2",
                params![company_id, secret_id],
            )
            .expect("delete secret");
        affected > 0
    }
}

/// A SQLite-backed [`WorkspaceRepository`] — durable, company-scoped workspaces.
#[derive(Clone)]
pub struct SqliteWorkspaceStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteWorkspaceStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                issue_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'starting'
            )",
            [],
        )
        .expect("create workspaces table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl WorkspaceRepository for SqliteWorkspaceStore {
    fn create(&self, company_id: &str, input: CreateWorkspace) -> Workspace {
        let workspace = Workspace {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            issue_id: input.issue_id,
            status: input.status,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO workspaces (id, company_id, issue_id, status)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    workspace.id,
                    workspace.company_id,
                    workspace.issue_id,
                    workspace.status
                ],
            )
            .expect("insert workspace");
        workspace
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Workspace> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, issue_id, status FROM workspaces
                 WHERE company_id = ?1 ORDER BY id",
            )
            .expect("prepare list workspaces");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(Workspace {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    issue_id: row.get(2)?,
                    status: row.get(3)?,
                })
            })
            .expect("query workspaces");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect workspaces")
    }

    fn update(
        &self,
        company_id: &str,
        workspace_id: &str,
        patch: UpdateWorkspace,
    ) -> Option<Workspace> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, issue_id, status FROM workspaces
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, workspace_id],
                |row| {
                    Ok(Workspace {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        issue_id: row.get(2)?,
                        status: row.get(3)?,
                    })
                },
            )
            .optional()
            .expect("query workspace");
        let mut workspace = current?;
        if let Some(status) = patch.status {
            workspace.status = status;
        }
        if let Some(issue_id) = patch.issue_id {
            workspace.issue_id = issue_id;
        }
        conn.execute(
            "UPDATE workspaces SET issue_id = ?1, status = ?2 WHERE company_id = ?3 AND id = ?4",
            params![
                workspace.issue_id,
                workspace.status,
                company_id,
                workspace_id
            ],
        )
        .expect("update workspace");
        Some(workspace)
    }

    fn delete(&self, company_id: &str, workspace_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM workspaces WHERE company_id = ?1 AND id = ?2",
                params![company_id, workspace_id],
            )
            .expect("delete workspace");
        affected > 0
    }
}

/// A SQLite-backed [`PluginRepository`] — durable, company-scoped plugin registry.
#[derive(Clone)]
pub struct SqlitePluginStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqlitePluginStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS plugins (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                plugin_id TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1
            )",
            [],
        )
        .expect("create plugins table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl PluginRepository for SqlitePluginStore {
    fn create(&self, company_id: &str, input: InstallPlugin) -> CompanyPlugin {
        let plugin = CompanyPlugin {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            plugin_id: input.plugin_id,
            enabled: input.enabled,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO plugins (id, company_id, plugin_id, enabled)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    plugin.id,
                    plugin.company_id,
                    plugin.plugin_id,
                    plugin.enabled
                ],
            )
            .expect("insert plugin");
        plugin
    }

    fn list_by_company(&self, company_id: &str) -> Vec<CompanyPlugin> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, plugin_id, enabled FROM plugins
                 WHERE company_id = ?1 ORDER BY plugin_id",
            )
            .expect("prepare list plugins");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(CompanyPlugin {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    plugin_id: row.get(2)?,
                    enabled: row.get(3)?,
                })
            })
            .expect("query plugins");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect plugins")
    }

    fn update(
        &self,
        company_id: &str,
        plugin_id: &str,
        patch: UpdatePlugin,
    ) -> Option<CompanyPlugin> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, plugin_id, enabled FROM plugins
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, plugin_id],
                |row| {
                    Ok(CompanyPlugin {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        plugin_id: row.get(2)?,
                        enabled: row.get(3)?,
                    })
                },
            )
            .optional()
            .expect("query plugin");
        let mut plugin = current?;
        if let Some(enabled) = patch.enabled {
            plugin.enabled = enabled;
        }
        conn.execute(
            "UPDATE plugins SET enabled = ?1 WHERE company_id = ?2 AND id = ?3",
            params![plugin.enabled, company_id, plugin_id],
        )
        .expect("update plugin");
        Some(plugin)
    }

    fn delete(&self, company_id: &str, plugin_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM plugins WHERE company_id = ?1 AND id = ?2",
                params![company_id, plugin_id],
            )
            .expect("delete plugin");
        affected > 0
    }
}

/// A SQLite-backed [`AdapterRepository`] — durable, company-scoped adapter registry.
#[derive(Clone)]
pub struct SqliteAdapterStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteAdapterStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS adapters (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                adapter_type TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1
            )",
            [],
        )
        .expect("create adapters table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl AdapterRepository for SqliteAdapterStore {
    fn create(&self, company_id: &str, input: ConfigureAdapter) -> CompanyAdapter {
        let adapter = CompanyAdapter {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            adapter_type: input.adapter_type,
            enabled: input.enabled,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO adapters (id, company_id, adapter_type, enabled)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    adapter.id,
                    adapter.company_id,
                    adapter.adapter_type,
                    adapter.enabled
                ],
            )
            .expect("insert adapter");
        adapter
    }

    fn list_by_company(&self, company_id: &str) -> Vec<CompanyAdapter> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, company_id, adapter_type, enabled FROM adapters
                 WHERE company_id = ?1 ORDER BY adapter_type",
            )
            .expect("prepare list adapters");
        let rows = stmt
            .query_map(params![company_id], |row| {
                Ok(CompanyAdapter {
                    id: row.get(0)?,
                    company_id: row.get(1)?,
                    adapter_type: row.get(2)?,
                    enabled: row.get(3)?,
                })
            })
            .expect("query adapters");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect adapters")
    }

    fn update(
        &self,
        company_id: &str,
        adapter_id: &str,
        patch: UpdateAdapter,
    ) -> Option<CompanyAdapter> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                "SELECT id, company_id, adapter_type, enabled FROM adapters
                 WHERE company_id = ?1 AND id = ?2",
                params![company_id, adapter_id],
                |row| {
                    Ok(CompanyAdapter {
                        id: row.get(0)?,
                        company_id: row.get(1)?,
                        adapter_type: row.get(2)?,
                        enabled: row.get(3)?,
                    })
                },
            )
            .optional()
            .expect("query adapter");
        let mut adapter = current?;
        if let Some(enabled) = patch.enabled {
            adapter.enabled = enabled;
        }
        conn.execute(
            "UPDATE adapters SET enabled = ?1 WHERE company_id = ?2 AND id = ?3",
            params![adapter.enabled, company_id, adapter_id],
        )
        .expect("update adapter");
        Some(adapter)
    }

    fn delete(&self, company_id: &str, adapter_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM adapters WHERE company_id = ?1 AND id = ?2",
                params![company_id, adapter_id],
            )
            .expect("delete adapter");
        affected > 0
    }
}

/// A SQLite-backed [`GoalRepository`] — durable, company-scoped goals.
#[derive(Clone)]
pub struct SqliteGoalStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteGoalStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS goals (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                level TEXT NOT NULL DEFAULT 'task',
                status TEXT NOT NULL DEFAULT 'planned',
                parent_id TEXT,
                owner_agent_id TEXT
            )",
            [],
        )
        .expect("create goals table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

fn row_to_goal(row: &Row) -> rusqlite::Result<Goal> {
    Ok(Goal {
        id: row.get(0)?,
        company_id: row.get(1)?,
        title: row.get(2)?,
        description: row.get(3)?,
        level: row.get(4)?,
        status: row.get(5)?,
        parent_id: row.get(6)?,
        owner_agent_id: row.get(7)?,
    })
}

const GOAL_COLUMNS: &str =
    "id, company_id, title, description, level, status, parent_id, owner_agent_id";

impl GoalRepository for SqliteGoalStore {
    fn create(&self, company_id: &str, input: CreateGoal) -> Goal {
        let goal = Goal {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            title: input.title,
            description: input.description,
            level: input.level,
            status: input.status,
            parent_id: input.parent_id,
            owner_agent_id: input.owner_agent_id,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO goals (id, company_id, title, description, level, status, parent_id, owner_agent_id)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    goal.id,
                    goal.company_id,
                    goal.title,
                    goal.description,
                    goal.level,
                    goal.status,
                    goal.parent_id,
                    goal.owner_agent_id,
                ],
            )
            .expect("insert goal");
        goal
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Goal> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(&format!(
                "SELECT {GOAL_COLUMNS} FROM goals WHERE company_id = ?1 ORDER BY title"
            ))
            .expect("prepare list goals");
        let rows = stmt
            .query_map(params![company_id], row_to_goal)
            .expect("query goals");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect goals")
    }

    fn update(&self, company_id: &str, goal_id: &str, patch: UpdateGoal) -> Option<Goal> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                &format!("SELECT {GOAL_COLUMNS} FROM goals WHERE company_id = ?1 AND id = ?2"),
                params![company_id, goal_id],
                row_to_goal,
            )
            .optional()
            .expect("query goal");
        let mut goal = current?;
        if let Some(title) = patch.title {
            goal.title = title;
        }
        if let Some(description) = patch.description {
            goal.description = Some(description);
        }
        if let Some(level) = patch.level {
            goal.level = level;
        }
        if let Some(status) = patch.status {
            goal.status = status;
        }
        if let Some(parent_id) = patch.parent_id {
            goal.parent_id = Some(parent_id);
        }
        if let Some(owner_agent_id) = patch.owner_agent_id {
            goal.owner_agent_id = Some(owner_agent_id);
        }
        conn.execute(
            "UPDATE goals SET title = ?1, description = ?2, level = ?3, status = ?4,
                 parent_id = ?5, owner_agent_id = ?6 WHERE company_id = ?7 AND id = ?8",
            params![
                goal.title,
                goal.description,
                goal.level,
                goal.status,
                goal.parent_id,
                goal.owner_agent_id,
                company_id,
                goal_id,
            ],
        )
        .expect("update goal");
        Some(goal)
    }

    fn delete(&self, company_id: &str, goal_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM goals WHERE company_id = ?1 AND id = ?2",
                params![company_id, goal_id],
            )
            .expect("delete goal");
        affected > 0
    }
}

/// A SQLite-backed [`ActivityRepository`] — durable, company-scoped audit feed.
#[derive(Clone)]
pub struct SqliteActivityStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteActivityStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS activity (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                actor_type TEXT NOT NULL DEFAULT 'system',
                actor_id TEXT NOT NULL,
                action TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                agent_id TEXT,
                run_id TEXT,
                details TEXT
            )",
            [],
        )
        .expect("create activity table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

fn row_to_activity(row: &Row) -> rusqlite::Result<ActivityEvent> {
    let details: Option<String> = row.get(9)?;
    Ok(ActivityEvent {
        id: row.get(0)?,
        company_id: row.get(1)?,
        actor_type: row.get(2)?,
        actor_id: row.get(3)?,
        action: row.get(4)?,
        entity_type: row.get(5)?,
        entity_id: row.get(6)?,
        agent_id: row.get(7)?,
        run_id: row.get(8)?,
        details: details.map(|s| serde_json::from_str(&s).expect("parse activity details")),
    })
}

const ACTIVITY_COLUMNS: &str =
    "id, company_id, actor_type, actor_id, action, entity_type, entity_id, agent_id, run_id, details";

impl ActivityRepository for SqliteActivityStore {
    fn create(&self, company_id: &str, input: CreateActivity) -> ActivityEvent {
        let event = ActivityEvent {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            actor_type: input.actor_type,
            actor_id: input.actor_id,
            action: input.action,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            agent_id: input.agent_id,
            run_id: None,
            details: input.details,
        };
        let details_text = event
            .details
            .as_ref()
            .map(|v| serde_json::to_string(v).expect("serialize activity details"));
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO activity (id, company_id, actor_type, actor_id, action, entity_type, entity_id, agent_id, run_id, details)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    event.id,
                    event.company_id,
                    event.actor_type,
                    event.actor_id,
                    event.action,
                    event.entity_type,
                    event.entity_id,
                    event.agent_id,
                    event.run_id,
                    details_text,
                ],
            )
            .expect("insert activity");
        event
    }

    fn list(&self, company_id: &str, filters: &ActivityFilters) -> Vec<ActivityEvent> {
        // Clamp the limit to the same bounds as the in-memory store (default 100,
        // max 500). rowid DESC yields newest-first without a timestamp column.
        let limit = filters.limit.map(|n| n.clamp(1, 500)).unwrap_or(100) as i64;
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(&format!(
                "SELECT {ACTIVITY_COLUMNS} FROM activity
                 WHERE company_id = ?1
                   AND (?2 IS NULL OR agent_id = ?2)
                   AND (?3 IS NULL OR entity_type = ?3)
                   AND (?4 IS NULL OR entity_id = ?4)
                 ORDER BY rowid DESC
                 LIMIT ?5"
            ))
            .expect("prepare list activity");
        let rows = stmt
            .query_map(
                params![
                    company_id,
                    filters.agent_id,
                    filters.entity_type,
                    filters.entity_id,
                    limit,
                ],
                row_to_activity,
            )
            .expect("query activity");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect activity")
    }
}

/// A SQLite-backed [`EnvironmentRepository`] — durable, company-scoped execution
/// environments. JSON `config`/`envVars`/`metadata` are stored as TEXT.
#[derive(Clone)]
pub struct SqliteEnvironmentStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteEnvironmentStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS environments (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                driver TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active',
                config TEXT NOT NULL DEFAULT '{}',
                env_vars TEXT NOT NULL DEFAULT '{}',
                metadata TEXT
            )",
            [],
        )
        .expect("create environments table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

fn row_to_environment(row: &Row) -> rusqlite::Result<Environment> {
    let config: String = row.get(6)?;
    let env_vars: String = row.get(7)?;
    let metadata: Option<String> = row.get(8)?;
    Ok(Environment {
        id: row.get(0)?,
        company_id: row.get(1)?,
        name: row.get(2)?,
        description: row.get(3)?,
        driver: row.get(4)?,
        status: row.get(5)?,
        config: serde_json::from_str(&config).expect("parse environment config"),
        env_vars: serde_json::from_str(&env_vars).expect("parse environment envVars"),
        metadata: metadata.map(|s| serde_json::from_str(&s).expect("parse environment metadata")),
    })
}

const ENVIRONMENT_COLUMNS: &str =
    "id, company_id, name, description, driver, status, config, env_vars, metadata";

impl EnvironmentRepository for SqliteEnvironmentStore {
    fn create(&self, company_id: &str, input: CreateEnvironment) -> Environment {
        let environment = Environment {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            name: input.name,
            description: input.description,
            driver: input.driver,
            status: input.status,
            config: input.config,
            env_vars: input.env_vars,
            metadata: input.metadata,
        };
        let config_text = serde_json::to_string(&environment.config).expect("serialize config");
        let env_vars_text =
            serde_json::to_string(&environment.env_vars).expect("serialize envVars");
        let metadata_text = environment
            .metadata
            .as_ref()
            .map(|v| serde_json::to_string(v).expect("serialize metadata"));
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO environments (id, company_id, name, description, driver, status, config, env_vars, metadata)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    environment.id,
                    environment.company_id,
                    environment.name,
                    environment.description,
                    environment.driver,
                    environment.status,
                    config_text,
                    env_vars_text,
                    metadata_text,
                ],
            )
            .expect("insert environment");
        environment
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Environment> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(&format!(
                "SELECT {ENVIRONMENT_COLUMNS} FROM environments WHERE company_id = ?1 ORDER BY name"
            ))
            .expect("prepare list environments");
        let rows = stmt
            .query_map(params![company_id], row_to_environment)
            .expect("query environments");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect environments")
    }

    fn update(
        &self,
        company_id: &str,
        environment_id: &str,
        patch: UpdateEnvironment,
    ) -> Option<Environment> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                &format!(
                    "SELECT {ENVIRONMENT_COLUMNS} FROM environments WHERE company_id = ?1 AND id = ?2"
                ),
                params![company_id, environment_id],
                row_to_environment,
            )
            .optional()
            .expect("query environment");
        let mut environment = current?;
        if let Some(name) = patch.name {
            environment.name = name;
        }
        if let Some(description) = patch.description {
            environment.description = Some(description);
        }
        if let Some(driver) = patch.driver {
            environment.driver = driver;
        }
        if let Some(status) = patch.status {
            environment.status = status;
        }
        if let Some(config) = patch.config {
            environment.config = config;
        }
        if let Some(env_vars) = patch.env_vars {
            environment.env_vars = env_vars;
        }
        if let Some(metadata) = patch.metadata {
            environment.metadata = Some(metadata);
        }
        let config_text = serde_json::to_string(&environment.config).expect("serialize config");
        let env_vars_text =
            serde_json::to_string(&environment.env_vars).expect("serialize envVars");
        let metadata_text = environment
            .metadata
            .as_ref()
            .map(|v| serde_json::to_string(v).expect("serialize metadata"));
        conn.execute(
            "UPDATE environments SET name = ?1, description = ?2, driver = ?3, status = ?4,
                 config = ?5, env_vars = ?6, metadata = ?7 WHERE company_id = ?8 AND id = ?9",
            params![
                environment.name,
                environment.description,
                environment.driver,
                environment.status,
                config_text,
                env_vars_text,
                metadata_text,
                company_id,
                environment_id,
            ],
        )
        .expect("update environment");
        Some(environment)
    }

    fn delete(&self, company_id: &str, environment_id: &str) -> bool {
        let affected = self
            .conn
            .lock()
            .unwrap()
            .execute(
                "DELETE FROM environments WHERE company_id = ?1 AND id = ?2",
                params![company_id, environment_id],
            )
            .expect("delete environment");
        affected > 0
    }
}

/// A SQLite-backed [`CostRepository`] — durable, company-scoped cost events.
#[derive(Clone)]
pub struct SqliteCostStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCostStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cost_events (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                issue_id TEXT,
                provider TEXT NOT NULL,
                biller TEXT NOT NULL,
                billing_type TEXT NOT NULL DEFAULT 'unknown',
                model TEXT NOT NULL,
                input_tokens INTEGER NOT NULL DEFAULT 0,
                cached_input_tokens INTEGER NOT NULL DEFAULT 0,
                output_tokens INTEGER NOT NULL DEFAULT 0,
                cost_cents INTEGER NOT NULL,
                occurred_at TEXT NOT NULL
            )",
            [],
        )
        .expect("create cost_events table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl CostRepository for SqliteCostStore {
    fn create(&self, company_id: &str, input: CreateCostEvent) -> CostEvent {
        let biller = input.biller.unwrap_or_else(|| input.provider.clone());
        let event = CostEvent {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            agent_id: input.agent_id,
            issue_id: input.issue_id,
            provider: input.provider,
            biller,
            billing_type: input.billing_type,
            model: input.model,
            input_tokens: input.input_tokens,
            cached_input_tokens: input.cached_input_tokens,
            output_tokens: input.output_tokens,
            cost_cents: input.cost_cents,
            occurred_at: input.occurred_at,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO cost_events (id, company_id, agent_id, issue_id, provider, biller, billing_type, model, input_tokens, cached_input_tokens, output_tokens, cost_cents, occurred_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![
                    event.id,
                    event.company_id,
                    event.agent_id,
                    event.issue_id,
                    event.provider,
                    event.biller,
                    event.billing_type,
                    event.model,
                    event.input_tokens,
                    event.cached_input_tokens,
                    event.output_tokens,
                    event.cost_cents,
                    event.occurred_at,
                ],
            )
            .expect("insert cost event");
        event
    }

    fn spend_cents(&self, company_id: &str) -> i64 {
        self.conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT COALESCE(SUM(cost_cents), 0) FROM cost_events WHERE company_id = ?1",
                params![company_id],
                |row| row.get(0),
            )
            .expect("sum cost events")
    }
}

/// A SQLite-backed [`InboxRepository`] — durable, (company, user)-scoped inbox
/// dismissals. The primary key makes dismiss idempotent per item.
#[derive(Clone)]
pub struct SqliteInboxStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteInboxStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS inbox_dismissals (
                company_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                item_key TEXT NOT NULL,
                dismissed_at TEXT NOT NULL,
                PRIMARY KEY (company_id, user_id, item_key)
            )",
            [],
        )
        .expect("create inbox_dismissals table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl InboxRepository for SqliteInboxStore {
    fn dismiss(
        &self,
        company_id: &str,
        user_id: &str,
        item_key: &str,
        dismissed_at: String,
    ) -> InboxDismissal {
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO inbox_dismissals (company_id, user_id, item_key, dismissed_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(company_id, user_id, item_key)
                 DO UPDATE SET dismissed_at = excluded.dismissed_at",
                params![company_id, user_id, item_key, dismissed_at],
            )
            .expect("upsert inbox dismissal");
        InboxDismissal {
            company_id: company_id.to_string(),
            user_id: user_id.to_string(),
            item_key: item_key.to_string(),
            dismissed_at,
        }
    }

    fn list(&self, company_id: &str, user_id: &str) -> Vec<InboxDismissal> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT company_id, user_id, item_key, dismissed_at FROM inbox_dismissals
                 WHERE company_id = ?1 AND user_id = ?2 ORDER BY item_key",
            )
            .expect("prepare list inbox dismissals");
        let rows = stmt
            .query_map(params![company_id, user_id], |row| {
                Ok(InboxDismissal {
                    company_id: row.get(0)?,
                    user_id: row.get(1)?,
                    item_key: row.get(2)?,
                    dismissed_at: row.get(3)?,
                })
            })
            .expect("query inbox dismissals");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect inbox dismissals")
    }
}

/// A SQLite-backed [`PipelineRepository`] — durable, company-scoped pipeline
/// entities (stages/cases remain in-memory for now).
#[derive(Clone)]
pub struct SqlitePipelineStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqlitePipelineStore {
    /// Opens (or creates) the database at `path` and ensures the schema exists.
    pub fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pipelines (
                id TEXT PRIMARY KEY,
                company_id TEXT NOT NULL,
                key TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                project_id TEXT,
                enforce_transitions INTEGER NOT NULL DEFAULT 0,
                archived INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .expect("create pipelines table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

fn row_to_pipeline(row: &Row) -> rusqlite::Result<Pipeline> {
    Ok(Pipeline {
        id: row.get(0)?,
        company_id: row.get(1)?,
        key: row.get(2)?,
        name: row.get(3)?,
        description: row.get(4)?,
        project_id: row.get(5)?,
        enforce_transitions: row.get(6)?,
        archived: row.get(7)?,
    })
}

const PIPELINE_COLUMNS: &str =
    "id, company_id, key, name, description, project_id, enforce_transitions, archived";

impl PipelineRepository for SqlitePipelineStore {
    fn create(&self, company_id: &str, input: CreatePipeline) -> Pipeline {
        let pipeline = Pipeline {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.to_string(),
            key: input.key,
            name: input.name,
            description: input.description,
            project_id: input.project_id,
            enforce_transitions: input.enforce_transitions,
            archived: false,
        };
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO pipelines (id, company_id, key, name, description, project_id, enforce_transitions, archived)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    pipeline.id,
                    pipeline.company_id,
                    pipeline.key,
                    pipeline.name,
                    pipeline.description,
                    pipeline.project_id,
                    pipeline.enforce_transitions,
                    pipeline.archived,
                ],
            )
            .expect("insert pipeline");
        pipeline
    }

    fn list_by_company(&self, company_id: &str) -> Vec<Pipeline> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(&format!(
                "SELECT {PIPELINE_COLUMNS} FROM pipelines WHERE company_id = ?1 ORDER BY key"
            ))
            .expect("prepare list pipelines");
        let rows = stmt
            .query_map(params![company_id], row_to_pipeline)
            .expect("query pipelines");
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect pipelines")
    }

    fn get(&self, company_id: &str, pipeline_id: &str) -> Option<Pipeline> {
        self.conn
            .lock()
            .unwrap()
            .query_row(
                &format!(
                    "SELECT {PIPELINE_COLUMNS} FROM pipelines WHERE company_id = ?1 AND id = ?2"
                ),
                params![company_id, pipeline_id],
                row_to_pipeline,
            )
            .optional()
            .expect("query pipeline")
    }

    fn update(
        &self,
        company_id: &str,
        pipeline_id: &str,
        patch: UpdatePipeline,
    ) -> Option<Pipeline> {
        let conn = self.conn.lock().unwrap();
        let current = conn
            .query_row(
                &format!(
                    "SELECT {PIPELINE_COLUMNS} FROM pipelines WHERE company_id = ?1 AND id = ?2"
                ),
                params![company_id, pipeline_id],
                row_to_pipeline,
            )
            .optional()
            .expect("query pipeline");
        let mut pipeline = current?;
        if let Some(name) = patch.name {
            pipeline.name = name;
        }
        if let Some(description) = patch.description {
            pipeline.description = Some(description);
        }
        if let Some(enforce_transitions) = patch.enforce_transitions {
            pipeline.enforce_transitions = enforce_transitions;
        }
        if let Some(archived) = patch.archived {
            pipeline.archived = archived;
        }
        conn.execute(
            "UPDATE pipelines SET name = ?1, description = ?2, enforce_transitions = ?3,
                 archived = ?4 WHERE company_id = ?5 AND id = ?6",
            params![
                pipeline.name,
                pipeline.description,
                pipeline.enforce_transitions,
                pipeline.archived,
                company_id,
                pipeline_id,
            ],
        )
        .expect("update pipeline");
        Some(pipeline)
    }
}
