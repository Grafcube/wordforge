export async function getName() {
    const res = await fetch("/api/v1/validate");
    const text = await res.text();
    if (res.ok) {
        return text;
    } else {
        throw new Error(text);
    }
}

export async function validate(target: string) {
    try {
        await getName();
    } catch (e: any) {
        location.href = target;
    }
}
