CREATE TABLE permissions (
    id smallint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    permission_name text UNIQUE NOT NULL
);

CREATE TABLE roles (
    id smallint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    role_name text UNIQUE NOT NULL
);

-- many to many table
CREATE TABLE roles_permissions (
    role_id smallint NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id smallint NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- all the emails collected for any reason
CREATE TABLE emails (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- email should not be arbitrarily long
    email varchar(127) UNIQUE NOT NULL,
    -- NULL means is not confirmed yet, use redis to store confirmation token
    email_confirmed_at timestamptz,
    -- if false then only send the minimal amount required of email
    subscribed boolean DEFAULT FALSE NOT NULL,
    active boolean DEFAULT TRUE NOT NULL,
    -- `created_at` should be read only
    created_at timestamptz DEFAULT transaction_timestamp() NOT NULL,
    updated_at timestamptz DEFAULT transaction_timestamp() NOT NULL
);

-- 'image/jpeg', 'image/png', etc
CREATE TABLE image_mime_types (
    id smallint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    mime text UNIQUE NOT NULL
);

-- `image_files` should be read only, images shouldn't be modified
-- if you want to update an image, please replace it with a new one
CREATE TABLE image_files (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- all images should have width and height to avoid FOUC
    width_px integer NOT NULL,
    height_px integer NOT NULL,
    -- image_uuid/image_file_uuid.{jpg|png}
    file_path text UNIQUE NOT NULL,
    size_bytes integer NOT NULL,
    mime_id smallint NOT NULL REFERENCES image_mime_types(id),
    -- `created_at` should be read only
    created_at timestamptz DEFAULT transaction_timestamp() NOT NULL
);

-- all the images used for any reason
CREATE TABLE images (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- limited to 35 characters for SEO reasons
    title varchar(35) UNIQUE NOT NULL,
    -- mandatory and limited to 127 characters for SEO reasons
    alt_text varchar(127) NOT NULL,
    -- optional and should not be arbitrarily long
    caption varchar(255),
    -- all images should have 3 sizes for performance reasons
    -- if the image is small, they could reference the same file
    small_file_id uuid NOT NULL REFERENCES image_files(id),
    medium_file_id uuid NOT NULL REFERENCES image_files(id),
    large_file_id uuid NOT NULL REFERENCES image_files(id),
    -- `created_at` should be read only
    created_at timestamptz DEFAULT transaction_timestamp() NOT NULL,
    updated_at timestamptz DEFAULT transaction_timestamp() NOT NULL
);

-- admins, members, and everyone else can be inside this table
CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- ascii-no-space only, mandatory
    username varchar(31) UNIQUE NOT NULL,
    -- user has the option to show full name and doesn't have to be UNIQUE
    full_name varchar(127),
    -- user has the option to upload profile picture
    profile_pic_id uuid REFERENCES images(id),
    email_id uuid UNIQUE NOT NULL,
    -- argon or better in PHC string format, use redis to store change passwd token
    passwd_hash text NOT NULL,
    active boolean DEFAULT TRUE NOT NULL,
    -- `created_at` should be read only
    created_at timestamptz DEFAULT transaction_timestamp() NOT NULL,
    updated_at timestamptz DEFAULT transaction_timestamp() NOT NULL
);

-- many to many table
CREATE TABLE users_roles (
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id smallint NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

-- blog posts
CREATE TABLE posts (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- limited to 100 characters for SEO reasons
    title varchar(100) UNIQUE NOT NULL,
    -- limited to 35 characters for SEO reasons
    short_title varchar(35) UNIQUE NOT NULL,
    -- `slug` is the ascii-no-space version of `short_title`
    slug text UNIQUE NOT NULL,
    -- limited to 150 characters for SEO reasons
    description varchar(150) NOT NULL,
    content text NOT NULL,
    autor_id uuid NOT NULL REFERENCES users(id),
    -- image shown inside the post
    cover_image_id uuid NOT NULL REFERENCES images(id),
    -- image shown when sharing link
    og_image_id uuid NOT NULL REFERENCES images(id),
    -- NULL until published
    published_at timestamptz,
    -- publishing a post requires confirmation
    active boolean DEFAULT FALSE NOT NULL,
    -- `created_at` should be read only
    created_at timestamptz DEFAULT transaction_timestamp() NOT NULL,
    updated_at timestamptz DEFAULT transaction_timestamp() NOT NULL
);

CREATE TABLE tags (
    id smallint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    -- `title` should not be arbitrarily long
    title varchar(31) UNIQUE NOT NULL,
    -- `slug` is the ascii-no-space version of `title`
    slug text UNIQUE NOT NULL
);

-- many to many table
CREATE TABLE posts_tags (
    post_id uuid NOT NULL REFERENCES posts(id),
    tag_id smallint NOT NULL REFERENCES tags(id),
    PRIMARY KEY (post_id, tag_id)
);