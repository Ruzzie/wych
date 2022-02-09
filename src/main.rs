use std::borrow::Borrow;
use std::error::Error;
use std::fs;
use std::path::{Path};
use std::time::{SystemTime, SystemTimeError};
use clap::Parser;
use colored::Colorize;
use gethostname::gethostname;
use wych::{AppVersion, TemplateEngine, Templates};


/// Generate an AppVersion.elm Elm file before compiling elm
///  so that you have your version info available in your  compiledelm app.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The output path to write the AppVersion.elm file to
    path: String,

    /// A string that represents this builds unique version
    ///
    /// AppVeyor: APPVEYOR_BUILD_VERSION
    /// Azure Pipelines: Build.BuildNumber
    //#[clap(short, long)] // when we omit this it is a positional argument
    #[clap(short, long,
    env = "APPVEYOR_BUILD_VERSION", // Appveyor
    env = "BUILD_BUILDNUMBER", // Azure pipelines
    )]
    version_string: String,

    /// The build number (counter)
    #[clap(short, long)]
    build_number: u32,

    /// The commit hash / changeset reference of the build to include in the version file.
    ///
    /// AppVeyor: APPVEYOR_REPO_COMMIT
    /// Azure Pipelines: BUILD_SOURCEVERSION
    #[clap(short, long,
    env = "APPVEYOR_REPO_COMMIT", // Appveyor
    env = "BUILD_SOURCEVERSION", // Azure pipelines
    )
    ]
    commit: Option<String>,

    /// The source process / environment that generated the AppVersion file.
    ///
    ///   This can be useful to see if the source was the build server / master branch or a test process etc.
    ///   When omitted source will be set automatically.
    #[clap(short, long)]
    source: Option<String>,

    /*/// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,*/
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}            ", "┬ ┬┬ ┬┌─┐┬ ┬".purple());
    println!("{}   {}       ", "│││└┬┘│  ├─┤".purple(), "A tiny tool for generating elm version files".blue());
    println!("{}          {}", "└┴┘ ┴ └─┘┴ ┴".purple(), "- by Ruzzie, since 2022 -".bright_green());
    println!("");

    let args = Args::parse();

    let mut hash: String = String::from("");

    match args.commit {
        Some(commit_str) => {
            hash = commit_str;
            println!("\t commit '{}'", hash)
        }
        None => println!("\t no commit info given :(")
    }


    let mut ts: u128 = 0;
    match as_unix_ts_millis(SystemTime::now()) {
        Ok(timestamp) => {
            ts = timestamp;
            println!("\t timestamp ms {}", timestamp)
        }
        Err(err) => eprintln!("error while getting ts {}", err.to_string())
    };

    let mut source = String::from("");

    match args.source {
        Some(source_str) => {
            source = source_str;
            println!("Source {}", source)
        }
        None => {
            let auto_source_str = format!("{}, {}", gethostname().to_string_lossy(), os_info::get().to_string());

            println!("No source given we put: '{}' there.", auto_source_str)
        }
    }


    let app_version_model = AppVersion {
        version: args.version_string,
        build_number: args.build_number,
        hash,
        timestamp: ts as i64,
        source,
    };

    let output_path = Path::new(&args.path).join("AppVersion.elm");

    let path_str = output_path.to_string_lossy().to_string();

    let templates = Templates::load();
    let template_engine = TemplateEngine::new(templates.borrow())?;

    let file_contents = template_engine.render(app_version_model)?;

    fs::write(output_path, file_contents)?;
    println!("successfully written file {}", path_str);
    Ok(())
}

fn as_unix_ts_millis(now: SystemTime) -> Result<u128, SystemTimeError>
{
    return now.duration_since(SystemTime::UNIX_EPOCH).map(|elapsed| elapsed.as_millis());
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}