use criterion::{criterion_group, criterion_main, Criterion};

use duxcore::prelude::Host;
use duxcore::prelude::tasklist_parser;


fn bench_yaml_tasklist_building(c: &mut Criterion) {
    c.bench_function("bench_yaml_tasklist_building", |b| {
        b.iter(|| {
          let host = Host::from_string("localhost".to_string());
          let yaml_raw_content = String::from(
"---
- name: Prerequisites
  steps:
    - name: 1. Test SSH connectivity
      ping:

    - name: 2. Upgrade the whole system
      with_sudo: true
      apt:
        upgrade: true

    - name: 3. Install git
      with_sudo: true
      apt:
        state: present
        package: git
    
    - name: 4. Clean before clone
      command:
        content: rm -rf dux

    - name: 5. Clone a repository
      command:
       content: git clone https://gitlab.com/dux-tool/dux-standard.git"
);
            std::hint::black_box(
        tasklist_parser(yaml_raw_content, &host)
            );
        });
    });
}

fn bench_json_tasklist_building(c: &mut Criterion) {
  c.bench_function("bench_json_tasklist_building", |b| {
      b.iter(|| {
        let host = Host::from_string("localhost".to_string());
        let json_raw_content = String::from(
"[
  {
    \"name\": \"Prerequisites\",
    \"steps\": [
      {
        \"name\": \"1. Test SSH connectivity\",
        \"ping\": null
      },
      {
        \"name\": \"2. Upgrade the whole system\",
        \"with_sudo\": true,
        \"apt\": {
          \"upgrade\": true
        }
      },
      {
        \"name\": \"3. Install git\",
        \"with_sudo\": true,
        \"apt\": {
          \"state\": \"present\",
          \"package\": \"git\"
        }
      },
      {
        \"name\": \"4. Clean before clone\",
        \"command\": {
          \"content\": \"rm -rf dux\"
        }
      },
      {
        \"name\": \"5. Clone a repository\",
        \"command\": {
          \"content\": \"git clone https://gitlab.com/dux-tool/dux-standard.git\"
        }
      }
    ]
  }
]"
);
          std::hint::black_box(
      tasklist_parser(json_raw_content, &host)
          );
      });
  });
}

criterion_group!(
    benches,
    bench_yaml_tasklist_building,
    bench_json_tasklist_building
);
criterion_main!(benches);
