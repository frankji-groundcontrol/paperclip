-- Phase 01 enum/default parity.
alter table paperclip.companies alter column require_board_approval_for_new_agents set default false;
alter type paperclip.agent_status rename value 'archived' to 'terminated';
alter type paperclip.agent_status add value if not exists 'idle';
alter type paperclip.agent_status add value if not exists 'running';
alter type paperclip.agent_status add value if not exists 'error';
alter type paperclip.approval_type add value if not exists 'budget_override_required';
alter type paperclip.approval_type add value if not exists 'request_board_approval';
alter type paperclip.approval_status add value if not exists 'revision_requested';
