-- Add migration script here

CREATE TYPE dp_attempt_status AS ENUM ('pending', 'passed', 'failed');

CREATE TABLE dp_attempt_events (
    attempt_event_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id VARCHAR(255) NOT NULL REFERENCES dp_users ON DELETE CASCADE,
    question_id BIGINT NOT NULL REFERENCES dp_questions ON DELETE CASCADE,
    query TEXT NOT NULL,
    status DP_ATTEMPT_STATUS NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX dp_attempt_events_user_id_idx ON dp_attempt_events (
    user_id
);
CREATE INDEX dp_attempt_events_question_id_idx ON dp_attempt_events (
    question_id
);
CREATE INDEX dp_attempt_events_user_question_id_idx ON dp_attempt_events (
    user_id, question_id
);

CREATE TABLE dp_solution_events (
    solution_event_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id VARCHAR(255) REFERENCES dp_users ON DELETE CASCADE,
    question_id BIGINT NOT NULL REFERENCES dp_questions ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX dp_solution_events_user_id_idx ON dp_solution_events (user_id);
CREATE INDEX dp_solution_events_question_id_idx ON dp_solution_events (
    question_id
);
CREATE INDEX dp_solution_events_user_question_id_idx ON dp_solution_events (
    user_id, question_id
);
