%global debug_package %{nil}
Name:           libinput-rs
Version:        0.1.0
Release:        1%{?dist}
Summary:        A memory-safe Rust replacement for the libinput stack

License:        MIT
URL:            https://github.com/Sisyphus/libinput-rs
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo, rust, systemd-devel, gcc
Requires:       systemd, udev



%description
A complete, high-performance optimization of the Linux input stack rewritten 
entirely in Rust.

%prep
%setup -q -c

%build
cargo build --release

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_libdir}
mkdir -p %{buildroot}%{_sysconfdir}/libinput-rs
mkdir -p %{buildroot}%{_unitdir}

install -m 0755 target/release/libinput-rs %{buildroot}%{_bindir}/libinput-rs
install -m 0644 src/config.json %{buildroot}%{_sysconfdir}/libinput-rs/config.json
install -m 0644 systemd/libinput-rs.service %{buildroot}%{_unitdir}/libinput-rs.service

%post
%systemd_post libinput-rs.service
udevadm control --reload-rules && udevadm trigger

%preun
%systemd_preun libinput-rs.service

%postun
%systemd_postun_with_restart libinput-rs.service

%files
%{_bindir}/libinput-rs
%{_sysconfdir}/libinput-rs/config.json
%{_unitdir}/libinput-rs.service

%changelog
* Mon Jun 29 2026 Sisyphus <sisyphus@sisyphuslinux.org> - 0.1.0-1
- Initial production release replacing legacy C-input implementations.
