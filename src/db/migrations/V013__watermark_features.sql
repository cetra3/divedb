
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'OverlayLocation') THEN
        CREATE TYPE "OverlayLocation" as enum ('TopLeft', 'TopRight', 'BottomLeft', 'BottomRight');
    END IF;
END$$;

alter table users add column watermark_location "OverlayLocation" not null default 'BottomRight';
alter table users add column copyright_location "OverlayLocation" default 'BottomLeft';