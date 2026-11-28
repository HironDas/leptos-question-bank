use std::time::Duration;

use icondata::Icon;
use leptos::prelude::*;
use singlestage::*;

const TOAST_DURATION: Duration = Duration::from_millis(4000);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ToastVariant {
    Success,
    Error,
    Info,
}

impl ToastVariant {
    fn icon(self) -> Icon {
        match self {
            ToastVariant::Success => icondata::LuCircleCheck,
            ToastVariant::Error => icondata::LuCircleX,
            ToastVariant::Info => icondata::LuInfo,
        }
    }

    fn icon_class(self) -> &'static str {
        match self {
            ToastVariant::Success => "text-green-600 dark:text-green-400",
            ToastVariant::Error => "text-(--destructive)",
            ToastVariant::Info => "text-blue-600 dark:text-blue-400",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Toast {
    id: u64,
    variant: ToastVariant,
    title: String,
    description: Option<String>,
}

#[derive(Clone, Copy)]
pub struct ToastContext {
    toasts: RwSignal<Vec<Toast>>,
    next_id: StoredValue<u64>,
}

impl ToastContext {
    pub fn show(&self, variant: ToastVariant, title: impl Into<String>, description: Option<String>) {
        let id = self.next_id.get_value();
        self.next_id.set_value(id + 1);

        self.toasts.update(|toasts| {
            toasts.push(Toast {
                id,
                variant,
                title: title.into(),
                description,
            })
        });

        let ctx = *self;
        set_timeout(move || ctx.dismiss(id), TOAST_DURATION);
    }

    pub fn success(&self, title: impl Into<String>) {
        self.show(ToastVariant::Success, title, None);
    }

    pub fn error(&self, title: impl Into<String>, description: impl Into<String>) {
        self.show(ToastVariant::Error, title, Some(description.into()));
    }

    pub fn info(&self, title: impl Into<String>) {
        self.show(ToastVariant::Info, title, None);
    }

    pub fn dismiss(&self, id: u64) {
        // try_update: the provider may already be disposed when the timeout fires
        self.toasts.try_update(|toasts| toasts.retain(|t| t.id != id));
    }
}

pub fn use_toast() -> ToastContext {
    expect_context::<ToastContext>()
}

#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let ctx = ToastContext {
        toasts: RwSignal::new(Vec::new()),
        next_id: StoredValue::new(0),
    };
    provide_context(ctx);

    view! {
        {children()}
        <div
            class="fixed bottom-4 right-4 z-50 flex w-[calc(100%-2rem)] max-w-sm flex-col gap-2"
            role="region"
            aria-live="polite"
        >
            <For
                each=move || ctx.toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    let id = toast.id;
                    let variant_icon = toast.variant.icon();
                    view! {
                        <div
                            role="status"
                            class="toast-enter flex items-start gap-3 rounded-lg border bg-(--background)
                            text-(--foreground) p-4 shadow-lg"
                        >
                            <span class=format!("mt-0.5 shrink-0 {}", toast.variant.icon_class())>
                                {icon!(variant_icon)}
                            </span>
                            <div class="flex-1 grid gap-1">
                                <p class="text-sm font-semibold">{toast.title}</p>
                                {toast.description.map(|d| view! {
                                    <p class="text-sm text-(--muted-foreground)">{d}</p>
                                })}
                            </div>
                            <button
                                type="button"
                                aria-label="Close"
                                class="shrink-0 opacity-60 hover:opacity-100"
                                on:click=move |_| ctx.dismiss(id)
                            >
                                {icon!(icondata::LuX)}
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}
